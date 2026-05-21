use std::collections::HashMap;

use crate::db::games::model::Game;
use crate::db::games::repository::OpeningAggregateRow;

use super::openings::{
    aggregate_openings_by_family, rows_to_family_map, signal_from_totals, OpeningFamilyTotals,
    OpeningSignal,
};
use super::types::{VersusGamePlan, VersusPlanEntry, VersusPlanSide};
use super::{GP_PLAN_ENTRIES_PER_LIST, GP_TIER1_MIN_GAMES, GP_TIER23_MIN_GAMES};

fn round_wr_pct(wr: f64) -> f64 {
    (wr * 10.0).round() / 10.0
}

fn mk_plan_entry(
    family: String,
    list_tier: &str,
    selection_tier: u8,
    reason_kind: &str,
    reason_params: HashMap<String, serde_json::Value>,
    self_wr: Option<f64>,
    opp_wr: Option<f64>,
    self_games: u32,
    opp_games: u32,
) -> VersusPlanEntry {
    VersusPlanEntry {
        title: family,
        selection_tier,
        reason_kind: reason_kind.to_string(),
        reason_params,
        tier: list_tier.to_string(),
        self_win_rate_pct: self_wr.map(round_wr_pct),
        opp_win_rate_pct: opp_wr.map(round_wr_pct),
        self_games,
        opp_games,
    }
}

fn tier1_gap_params(
    self_sig: OpeningSignal,
    opp_sig: OpeningSignal,
) -> HashMap<String, serde_json::Value> {
    let delta = self_sig.wr - opp_sig.wr;
    let mut m = HashMap::new();
    m.insert(
        "selfWr".into(),
        serde_json::json!(round_wr_pct(self_sig.wr)),
    );
    m.insert("oppWr".into(), serde_json::json!(round_wr_pct(opp_sig.wr)));
    m.insert("selfGames".into(), serde_json::json!(self_sig.total));
    m.insert("oppGames".into(), serde_json::json!(opp_sig.total));
    m.insert("delta".into(), serde_json::json!(delta.round() as i64));
    m
}

fn self_only_params(sig: OpeningSignal) -> HashMap<String, serde_json::Value> {
    let mut m = HashMap::new();
    m.insert("selfWr".into(), serde_json::json!(round_wr_pct(sig.wr)));
    m.insert("selfGames".into(), serde_json::json!(sig.total));
    m
}

fn opp_only_params(sig: OpeningSignal) -> HashMap<String, serde_json::Value> {
    let mut m = HashMap::new();
    m.insert("oppWr".into(), serde_json::json!(round_wr_pct(sig.wr)));
    m.insert("oppGames".into(), serde_json::json!(sig.total));
    m
}

fn entry_strength(e: &VersusPlanEntry) -> f64 {
    if let Some(delta) = e.reason_params.get("delta").and_then(|v| v.as_f64()) {
        return delta.abs();
    }
    if e.tier == "play" {
        e.self_win_rate_pct.unwrap_or(0.0)
    } else {
        100.0 - e.self_win_rate_pct.unwrap_or(100.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Play,
    Avoid,
}

fn sorted_top(mut candidates: Vec<(f64, VersusPlanEntry)>, dir: Direction) -> Vec<VersusPlanEntry> {
    candidates.sort_by(|a, b| {
        let ord = match dir {
            Direction::Play => b.0.partial_cmp(&a.0),
            Direction::Avoid => a.0.partial_cmp(&b.0),
        };
        ord.unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.1.title.cmp(&b.1.title))
    });
    candidates
        .into_iter()
        .take(GP_PLAN_ENTRIES_PER_LIST)
        .map(|(_, e)| e)
        .collect()
}

fn collect_tier1(
    self_by_family: &HashMap<String, OpeningFamilyTotals>,
    opp_by_family: &HashMap<String, OpeningFamilyTotals>,
    direction: Direction,
) -> Vec<VersusPlanEntry> {
    let (tier_label, reason_key) = match direction {
        Direction::Play => ("play", "tier1GapPlay"),
        Direction::Avoid => ("avoid", "tier1GapAvoid"),
    };
    let mut candidates: Vec<(f64, VersusPlanEntry)> = Vec::new();
    for (family, self_totals) in self_by_family {
        let self_sig = signal_from_totals(*self_totals);
        if self_sig.total < GP_TIER1_MIN_GAMES {
            continue;
        }
        let Some(opp_totals) = opp_by_family.get(family) else {
            continue;
        };
        let opp_sig = signal_from_totals(*opp_totals);
        if opp_sig.total < GP_TIER1_MIN_GAMES {
            continue;
        }
        let delta = self_sig.wr - opp_sig.wr;
        let keep = match direction {
            Direction::Play => delta > 0.0,
            Direction::Avoid => delta < 0.0,
        };
        if !keep {
            continue;
        }
        candidates.push((
            delta,
            mk_plan_entry(
                family.clone(),
                tier_label,
                1,
                reason_key,
                tier1_gap_params(self_sig, opp_sig),
                Some(self_sig.wr),
                Some(opp_sig.wr),
                self_sig.total.max(0) as u32,
                opp_sig.total.max(0) as u32,
            ),
        ));
    }
    sorted_top(candidates, direction)
}

fn collect_tier2(
    self_by_family: &HashMap<String, OpeningFamilyTotals>,
    opp_by_family: &HashMap<String, OpeningFamilyTotals>,
    direction: Direction,
) -> Vec<VersusPlanEntry> {
    let (tier_label, self_reason, opp_reason) = match direction {
        Direction::Play => ("play", "tier2SelfBest", "tier2OppWeak"),
        Direction::Avoid => ("avoid", "tier2SelfWorst", "tier2OppStrong"),
    };
    let prefer_higher_self = direction == Direction::Play;
    let prefer_higher_opp = direction == Direction::Avoid;

    let pick_best_self = |reason: &'static str, prefer_higher: bool| -> Option<VersusPlanEntry> {
        let mut best: Option<(f64, VersusPlanEntry)> = None;
        for (family, totals) in self_by_family {
            let sig = signal_from_totals(*totals);
            if sig.total < GP_TIER23_MIN_GAMES {
                continue;
            }
            let entry = mk_plan_entry(
                family.clone(),
                tier_label,
                2,
                reason,
                self_only_params(sig),
                Some(sig.wr),
                None,
                sig.total.max(0) as u32,
                0,
            );
            let take = best.as_ref().is_none_or(|(wr, _)| {
                if prefer_higher {
                    sig.wr > *wr
                } else {
                    sig.wr < *wr
                }
            });
            if take {
                best = Some((sig.wr, entry));
            }
        }
        best.map(|(_, e)| e)
    };

    let pick_best_opp = |reason: &'static str, prefer_higher: bool| -> Option<VersusPlanEntry> {
        let mut best: Option<(f64, VersusPlanEntry)> = None;
        for (family, totals) in opp_by_family {
            let sig = signal_from_totals(*totals);
            if sig.total < GP_TIER23_MIN_GAMES {
                continue;
            }
            let entry = mk_plan_entry(
                family.clone(),
                tier_label,
                2,
                reason,
                opp_only_params(sig),
                None,
                Some(sig.wr),
                0,
                sig.total.max(0) as u32,
            );
            let take = best.as_ref().is_none_or(|(wr, _)| {
                if prefer_higher {
                    sig.wr > *wr
                } else {
                    sig.wr < *wr
                }
            });
            if take {
                best = Some((sig.wr, entry));
            }
        }
        best.map(|(_, e)| e)
    };

    let mut out = Vec::new();
    if let Some(e) = pick_best_self(self_reason, prefer_higher_self) {
        out.push(e);
    }
    if let Some(e) = pick_best_opp(opp_reason, prefer_higher_opp) {
        if !out.iter().any(|x| x.title == e.title) {
            out.push(e);
        }
    }
    out.truncate(GP_PLAN_ENTRIES_PER_LIST);
    out
}

fn collect_tier3(
    self_by_family: &HashMap<String, OpeningFamilyTotals>,
    direction: Direction,
) -> Vec<VersusPlanEntry> {
    let (tier_label, reason) = match direction {
        Direction::Play => ("play", "tier3SelfTop"),
        Direction::Avoid => ("avoid", "tier3SelfBottom"),
    };
    let mut candidates: Vec<(f64, VersusPlanEntry)> = Vec::new();
    for (family, totals) in self_by_family {
        let sig = signal_from_totals(*totals);
        if sig.total < GP_TIER23_MIN_GAMES {
            continue;
        }
        candidates.push((
            sig.wr,
            mk_plan_entry(
                family.clone(),
                tier_label,
                3,
                reason,
                self_only_params(sig),
                Some(sig.wr),
                None,
                sig.total.max(0) as u32,
                0,
            ),
        ));
    }
    sorted_top(candidates, direction)
}

fn build_play_list(
    self_by_family: &HashMap<String, OpeningFamilyTotals>,
    opp_by_family: &HashMap<String, OpeningFamilyTotals>,
) -> Vec<VersusPlanEntry> {
    let tier1 = collect_tier1(self_by_family, opp_by_family, Direction::Play);
    if !tier1.is_empty() {
        return tier1;
    }
    let tier2 = collect_tier2(self_by_family, opp_by_family, Direction::Play);
    if !tier2.is_empty() {
        return tier2;
    }
    collect_tier3(self_by_family, Direction::Play)
}

fn build_avoid_list(
    self_by_family: &HashMap<String, OpeningFamilyTotals>,
    opp_by_family: &HashMap<String, OpeningFamilyTotals>,
) -> Vec<VersusPlanEntry> {
    let tier1 = collect_tier1(self_by_family, opp_by_family, Direction::Avoid);
    if !tier1.is_empty() {
        return tier1;
    }
    let tier2 = collect_tier2(self_by_family, opp_by_family, Direction::Avoid);
    if !tier2.is_empty() {
        return tier2;
    }
    collect_tier3(self_by_family, Direction::Avoid)
}

fn dedupe_plan_side(
    mut play: Vec<VersusPlanEntry>,
    mut avoid: Vec<VersusPlanEntry>,
) -> VersusPlanSide {
    let overlap: Vec<String> = play
        .iter()
        .filter(|p| avoid.iter().any(|a| a.title == p.title))
        .map(|p| p.title.clone())
        .collect();
    for family in overlap {
        let play_strength = play
            .iter()
            .find(|e| e.title == family)
            .map(entry_strength)
            .unwrap_or(0.0);
        let avoid_strength = avoid
            .iter()
            .find(|e| e.title == family)
            .map(entry_strength)
            .unwrap_or(0.0);
        if play_strength >= avoid_strength {
            avoid.retain(|e| e.title != family);
        } else {
            play.retain(|e| e.title != family);
        }
    }
    VersusPlanSide { play, avoid }
}

fn build_plan_side(
    self_by_family: &HashMap<String, OpeningFamilyTotals>,
    opp_by_family: &HashMap<String, OpeningFamilyTotals>,
) -> VersusPlanSide {
    let play = build_play_list(self_by_family, opp_by_family);
    let avoid = build_avoid_list(self_by_family, opp_by_family);
    dedupe_plan_side(play, avoid)
}

pub(crate) fn build_game_plan(
    opp_filtered: &[Game],
    _active_username: &str,
    self_white_agg: Vec<OpeningAggregateRow>,
    self_black_agg: Vec<OpeningAggregateRow>,
) -> VersusGamePlan {
    let self_white = rows_to_family_map(self_white_agg);
    let self_black = rows_to_family_map(self_black_agg);

    let opp_white = aggregate_openings_by_family(opp_filtered, Some("white"), false, None);
    let opp_black = aggregate_openings_by_family(opp_filtered, Some("black"), false, None);

    VersusGamePlan {
        opp_games_in_opening_slice: opp_filtered.len() as u32,
        as_white: build_plan_side(&self_white, &opp_black),
        as_black: build_plan_side(&self_black, &opp_white),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::db::games::model::Game;
    use crate::db::games::repository::OpeningAggregateRow;

    fn test_game(opening: &str, color: &str, result: &str, opponent: &str) -> Game {
        Game {
            id: "g1".into(),
            username: "opp".into(),
            platform: "Lichess".into(),
            rated: true,
            speed: "blitz".into(),
            time_control: "5+3".into(),
            created_at: 0,
            player_name: "opp".into(),
            player_id: "opp".into(),
            opponent_name: opponent.into(),
            opponent_id: opponent.into(),
            white_name: if color == "white" {
                "opp".into()
            } else {
                opponent.into()
            },
            white_id: "w".into(),
            black_name: if color == "black" {
                "opp".into()
            } else {
                opponent.into()
            },
            black_id: "b".into(),
            white_rating: None,
            black_rating: None,
            player_rating: Some(1500),
            opponent_rating: Some(1500),
            winner: None,
            player_color: color.into(),
            player_result: result.into(),
            opening_eco: None,
            opening_name: Some(opening.into()),
            moves: Some("e4".into()),
            last_fen: None,
            pgn: String::new(),
        }
    }

    #[test]
    fn game_plan_dedupes_family_into_one_tier() {
        let mut games = Vec::new();
        for _ in 0..5 {
            games.push(test_game("King's Indian Attack", "black", "loss", "random"));
        }
        let self_white = vec![OpeningAggregateRow {
            opening_name: "King's Indian Attack".into(),
            wins: 10,
            losses: 2,
            draws: 0,
            total: 12,
        }];
        let self_black = vec![OpeningAggregateRow {
            opening_name: "King's Indian Attack".into(),
            wins: 0,
            losses: 5,
            draws: 0,
            total: 5,
        }];
        let plan = build_game_plan(&games, "you", self_white, self_black);
        let titles_play: HashSet<_> = plan
            .as_white
            .play
            .iter()
            .map(|e| e.title.as_str())
            .collect();
        let titles_avoid: HashSet<_> = plan
            .as_white
            .avoid
            .iter()
            .map(|e| e.title.as_str())
            .collect();
        assert!(
            titles_play.intersection(&titles_avoid).count() == 0,
            "same family must not appear in play and avoid"
        );
    }

    #[test]
    fn game_plan_tier1_play_when_self_beats_opp_gap() {
        let games = (0..5)
            .map(|_| test_game("Lion Defense: Anti-Philidor", "black", "loss", "x"))
            .collect::<Vec<_>>();
        let self_white = vec![OpeningAggregateRow {
            opening_name: "Lion Defense".into(),
            wins: 6,
            losses: 0,
            draws: 0,
            total: 6,
        }];
        let plan = build_game_plan(&games, "you", self_white, vec![]);
        assert_eq!(plan.as_white.play.len(), 1);
        let entry = &plan.as_white.play[0];
        assert_eq!(entry.title, "Lion Defense");
        assert_eq!(entry.selection_tier, 1);
        assert_eq!(entry.reason_kind, "tier1GapPlay");
        assert!((entry.self_win_rate_pct.unwrap() - 100.0).abs() < 1e-9);
        assert!((entry.opp_win_rate_pct.unwrap() - 0.0).abs() < 1e-9);
        let delta = entry
            .reason_params
            .get("delta")
            .and_then(|v| v.as_i64())
            .unwrap();
        assert_eq!(delta, 100);
    }

    #[test]
    fn game_plan_tier1_avoid_when_opp_beats_self_gap() {
        let games = (0..5)
            .map(|_| test_game("Sicilian Defense: Closed", "black", "win", "x"))
            .collect::<Vec<_>>();
        let self_white = vec![OpeningAggregateRow {
            opening_name: "Sicilian Defense".into(),
            wins: 1,
            losses: 5,
            draws: 0,
            total: 6,
        }];
        let plan = build_game_plan(&games, "you", self_white, vec![]);
        assert_eq!(plan.as_white.avoid.len(), 1);
        let entry = &plan.as_white.avoid[0];
        assert_eq!(entry.title, "Sicilian Defense");
        assert_eq!(entry.selection_tier, 1);
        assert_eq!(entry.reason_kind, "tier1GapAvoid");
        let delta = entry
            .reason_params
            .get("delta")
            .and_then(|v| v.as_i64())
            .unwrap();
        assert!(delta < 0);
    }

    #[test]
    fn game_plan_tier2_play_when_only_self_has_enough_games() {
        let games: Vec<Game> = Vec::new();
        let self_white = vec![OpeningAggregateRow {
            opening_name: "English Opening".into(),
            wins: 4,
            losses: 1,
            draws: 0,
            total: 5,
        }];
        let plan = build_game_plan(&games, "you", self_white, vec![]);
        assert_eq!(plan.as_white.play.len(), 1);
        let entry = &plan.as_white.play[0];
        assert_eq!(entry.selection_tier, 2);
        assert_eq!(entry.reason_kind, "tier2SelfBest");
    }
}
