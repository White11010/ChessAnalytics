use std::collections::HashMap;

use crate::db::games::model::Game;
use crate::db::games::repository::OpeningAggregateRow;

use super::types::{VersusOpeningCard, VersusOpeningLineShare};
use super::MIN_OPENING_GAMES_SHOW;

/// Opening family totals: `(wins, draws, total)`.
pub(crate) type OpeningFamilyTotals = (i64, i64, i64);

#[derive(Debug, Clone, Copy)]
pub(crate) struct OpeningSignal {
    pub wr: f64,
    pub total: i64,
}

/// Opening "family" label: text before the first `:` (Lichess `Main: Variation` pattern), else full trimmed name.
pub(crate) fn opening_family_label(raw: &str) -> String {
    let s = raw.trim();
    let head = s.split_once(':').map(|(a, _)| a.trim()).unwrap_or(s);
    if head.is_empty() {
        s.to_string()
    } else {
        head.to_string()
    }
}

pub(crate) fn opening_score_pct(wins: i64, draws: i64, total: i64) -> f64 {
    if total <= 0 {
        return 0.0;
    }
    (100.0 * (wins as f64 + 0.5 * draws as f64) / total as f64).clamp(0.0, 100.0)
}

/// Groups per full `opening_name` into Lichess-style families (text before first `:`), returns merged totals plus child rows sorted by popularity.
pub(crate) fn group_rows_into_families(
    rows: Vec<OpeningAggregateRow>,
) -> Vec<(OpeningAggregateRow, Vec<OpeningAggregateRow>)> {
    let mut groups: HashMap<String, Vec<OpeningAggregateRow>> = HashMap::new();
    for r in rows {
        let key = opening_family_label(&r.opening_name);
        groups.entry(key).or_default().push(r);
    }
    groups
        .into_iter()
        .map(|(family_key, mut children)| {
            children.sort_by(|a, b| {
                b.total
                    .cmp(&a.total)
                    .then_with(|| a.opening_name.cmp(&b.opening_name))
            });
            let merged = children.iter().fold(
                OpeningAggregateRow {
                    opening_name: family_key,
                    wins: 0,
                    losses: 0,
                    draws: 0,
                    total: 0,
                },
                |mut acc, r| {
                    acc.wins += r.wins;
                    acc.losses += r.losses;
                    acc.draws += r.draws;
                    acc.total += r.total;
                    acc
                },
            );
            (merged, children)
        })
        .collect()
}

/// Per full `opening_name` from Lichess: `(wins, draws, total)`.
pub(crate) fn aggregate_openings(
    games: &[Game],
    player_color_filter: Option<&str>,
) -> HashMap<String, (i64, i64, i64)> {
    let mut m: HashMap<String, (i64, i64, i64)> = HashMap::new();
    for g in games {
        if player_color_filter.is_some_and(|c| g.player_color != c) {
            continue;
        }
        let Some(name_raw) = g.opening_name.as_ref().map(|s| s.trim().to_string()) else {
            continue;
        };
        if name_raw.is_empty() {
            continue;
        }
        let e = m.entry(name_raw).or_insert((0, 0, 0));
        e.2 += 1;
        match g.player_result.as_str() {
            "win" => e.0 += 1,
            "draw" => e.1 += 1,
            _ => {}
        }
    }
    m
}

pub(crate) fn rows_to_cards_with_lines(
    buckets: Vec<(OpeningAggregateRow, Vec<OpeningAggregateRow>)>,
    take: usize,
) -> Vec<VersusOpeningCard> {
    let mut v: Vec<VersusOpeningCard> = buckets
        .into_iter()
        .filter(|(merged, _)| merged.total >= MIN_OPENING_GAMES_SHOW)
        .map(|(merged, children)| {
            let fam_tot = merged.total.max(1) as f64;
            let lines: Vec<VersusOpeningLineShare> = children
                .iter()
                .map(|c| VersusOpeningLineShare {
                    name: c.opening_name.clone(),
                    total: c.total.max(0) as u32,
                    frequency_pct: (100.0 * c.total.max(0) as f64 / fam_tot).clamp(0.0, 100.0),
                })
                .collect();
            VersusOpeningCard {
                name: merged.opening_name.clone(),
                wins: merged.wins.max(0) as u32,
                draws: merged.draws.max(0) as u32,
                losses: merged.losses.max(0) as u32,
                total: merged.total.max(0) as u32,
                win_rate_pct: opening_score_pct(merged.wins, merged.draws, merged.total),
                lines,
            }
        })
        .collect();
    v.sort_by(|a, b| {
        b.total.cmp(&a.total).then_with(|| {
            b.win_rate_pct
                .partial_cmp(&a.win_rate_pct)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    });
    v.truncate(take);
    v
}

pub(crate) fn map_agg_hash_to_cards(
    m: HashMap<String, (i64, i64, i64)>,
    take: usize,
) -> Vec<VersusOpeningCard> {
    let rows: Vec<OpeningAggregateRow> = m
        .into_iter()
        .map(|(opening_name, (wins, draws, total))| {
            let losses = (total - wins - draws).max(0);
            OpeningAggregateRow {
                opening_name,
                wins,
                losses,
                draws,
                total,
            }
        })
        .collect();
    rows_to_cards_with_lines(group_rows_into_families(rows), take)
}

pub(crate) fn invert_player_result(result: &str) -> &str {
    match result {
        "win" => "loss",
        "loss" => "win",
        _ => "draw",
    }
}

/// Aggregates games into opening families; optionally filters H2H and inverts results to the active user's perspective.
pub(crate) fn aggregate_openings_by_family(
    games: &[Game],
    player_color_filter: Option<&str>,
    invert_result: bool,
    h2h_active_user: Option<&str>,
) -> HashMap<String, OpeningFamilyTotals> {
    let mut m: HashMap<String, OpeningFamilyTotals> = HashMap::new();
    for g in games {
        if player_color_filter.is_some_and(|c| g.player_color != c) {
            continue;
        }
        if let Some(user) = h2h_active_user {
            if !g.opponent_name.eq_ignore_ascii_case(user)
                && !g.opponent_id.eq_ignore_ascii_case(user)
            {
                continue;
            }
        }
        let Some(name_raw) = g.opening_name.as_ref() else {
            continue;
        };
        let family = opening_family_label(name_raw);
        if family.is_empty() {
            continue;
        }
        let e = m.entry(family).or_insert((0, 0, 0));
        e.2 += 1;
        let result = if invert_result {
            invert_player_result(g.player_result.as_str())
        } else {
            g.player_result.as_str()
        };
        match result {
            "win" => e.0 += 1,
            "draw" => e.1 += 1,
            _ => {}
        }
    }
    m
}

pub(crate) fn rows_to_family_map(rows: Vec<OpeningAggregateRow>) -> HashMap<String, OpeningFamilyTotals> {
    group_rows_into_families(rows)
        .into_iter()
        .map(|(merged, _)| {
            (
                merged.opening_name,
                (merged.wins, merged.draws, merged.total),
            )
        })
        .collect()
}

pub(crate) fn signal_from_totals(totals: OpeningFamilyTotals) -> OpeningSignal {
    let (wins, draws, total) = totals;
    OpeningSignal {
        wr: opening_score_pct(wins, draws, total),
        total,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::games::model::Game;

    #[test]
    fn opening_family_label_strips_variation() {
        assert_eq!(
            opening_family_label("  Modern Defense: Some Line  "),
            "Modern Defense"
        );
        assert_eq!(opening_family_label("French Defense"), "French Defense");
    }

    #[test]
    fn opening_score_pct_counts_half_draws() {
        assert!((opening_score_pct(1, 2, 4) - 50.0).abs() < 1e-9);
        assert!((opening_score_pct(2, 0, 4) - 50.0).abs() < 1e-9);
    }

    #[test]
    fn group_opening_rows_into_families_sums_and_line_shares() {
        let rows = vec![
            OpeningAggregateRow {
                opening_name: "Scandinavian Defense: Main Line".into(),
                wins: 2,
                losses: 1,
                draws: 1,
                total: 4,
            },
            OpeningAggregateRow {
                opening_name: "Scandinavian Defense: Mieses".into(),
                wins: 1,
                losses: 0,
                draws: 1,
                total: 2,
            },
        ];
        let buckets = group_rows_into_families(rows);
        assert_eq!(buckets.len(), 1);
        let (m, children) = &buckets[0];
        assert_eq!(m.opening_name, "Scandinavian Defense");
        assert_eq!(m.wins, 3);
        assert_eq!(m.draws, 2);
        assert_eq!(m.total, 6);
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].opening_name, "Scandinavian Defense: Main Line");
        assert_eq!(children[0].total, 4);
        assert_eq!(children[1].opening_name, "Scandinavian Defense: Mieses");
        assert_eq!(children[1].total, 2);
        assert!((opening_score_pct(m.wins, m.draws, m.total) - (100.0 * 4.0 / 6.0)).abs() < 1e-9);

        let cards = rows_to_cards_with_lines(buckets, 10);
        assert_eq!(cards.len(), 1);
        let c = &cards[0];
        assert_eq!(c.lines.len(), 2);
        assert_eq!(c.lines[0].name, "Scandinavian Defense: Main Line");
        assert_eq!(c.lines[0].total, 4);
        assert!((c.lines[0].frequency_pct - (100.0 * 4.0 / 6.0)).abs() < 1e-9);
        assert_eq!(c.lines[1].name, "Scandinavian Defense: Mieses");
        assert_eq!(c.lines[1].total, 2);
        assert!((c.lines[1].frequency_pct - (100.0 * 2.0 / 6.0)).abs() < 1e-9);
    }

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
    fn invert_player_result_flips_win_loss() {
        assert_eq!(invert_player_result("win"), "loss");
        assert_eq!(invert_player_result("loss"), "win");
        assert_eq!(invert_player_result("draw"), "draw");
    }

    #[test]
    fn h2h_aggregate_inverts_opponent_result() {
        let games = vec![
            test_game("Lion Defense: Anti-Philidor", "white", "loss", "you"),
            test_game("Lion Defense: Anti-Philidor", "white", "loss", "you"),
        ];
        let h2h = aggregate_openings_by_family(&games, Some("white"), true, Some("you"));
        let totals = h2h.get("Lion Defense").expect("family");
        assert_eq!(totals.0, 2, "user wins both from inverted perspective");
        assert_eq!(totals.2, 2);
    }
}
