//! Static population pentagon benchmarks by rating bucket; Versus and profile chart compare the player to these norms.
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub struct Pentagon {
    pub accuracy: f64,
    pub stability: f64,
    pub conversion: f64,
    pub openings: f64,
    pub endgame: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct PhaseErrorShares {
    pub opening: i64,
    pub middlegame: i64,
    pub endgame: i64,
}

#[derive(Debug, Deserialize)]
struct BucketRaw {
    label: String,
    #[serde(default)]
    acpl_avg: f64,
    pentagon: Pentagon,
    #[serde(default = "default_phase_error_opening")]
    phase_error_share_opening: f64,
    #[serde(default = "default_phase_error_middlegame")]
    phase_error_share_middlegame: f64,
    #[serde(default = "default_phase_error_endgame")]
    phase_error_share_endgame: f64,
}

fn default_phase_error_opening() -> f64 {
    16.0
}
fn default_phase_error_middlegame() -> f64 {
    28.0
}
fn default_phase_error_endgame() -> f64 {
    56.0
}

#[derive(Debug, Deserialize)]
struct LookupRule {
    #[serde(rename = "max")]
    max_rating: i64,
    bucket: String,
}

#[derive(Debug, Deserialize)]
struct BenchmarksFile {
    buckets: HashMap<String, BucketRaw>,
    lookup: LookupRules,
}

#[derive(Debug, Deserialize)]
struct LookupRules {
    rules: Vec<LookupRule>,
}

struct BenchmarksData {
    buckets: HashMap<String, BucketRaw>,
    // Pre-sorted so `bucket_key_for_rating` scans once without sorting on every call (hot path for UI).
    rules: Vec<(i64, String)>,
}

static DATA: OnceLock<BenchmarksData> = OnceLock::new();

fn data() -> &'static BenchmarksData {
    DATA.get_or_init(|| {
        let raw: BenchmarksFile = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/benchmarks.json"
        )))
        .expect("benchmarks.json must parse");

        let mut rules: Vec<(i64, String)> = raw
            .lookup
            .rules
            .into_iter()
            .map(|r| (r.max_rating, r.bucket))
            .collect();
        rules.sort_by_key(|(m, _)| *m);

        BenchmarksData {
            buckets: raw.buckets,
            rules,
        }
    })
}

/// Maps a rating to a benchmark bucket key using ascending `max_rating` rules from `benchmarks.json`.
pub fn bucket_key_for_rating(rating: i64) -> String {
    let d = data();
    for (mx, bucket) in &d.rules {
        if rating <= *mx {
            return bucket.clone();
        }
    }
    d.rules
        .last()
        .map(|(_, b)| b.clone())
        .unwrap_or_else(|| "under_800".to_string())
}

/// Returns the pentagon vertices and human label for a bucket key, or None if JSON data is missing that bucket.
pub fn pentagon_and_label(bucket_key: &str) -> Option<(Pentagon, String)> {
    let d = data();
    d.buckets.get(bucket_key).map(|b| (b.pentagon.clone(), b.label.clone()))
}

/// Population average centipawn loss per move for the player's rating bucket.
pub fn acpl_avg_for_rating(rating: i64) -> f64 {
    let key = bucket_key_for_rating(rating);
    let d = data();
    let b = d.buckets.get(&key).or_else(|| d.buckets.values().next());
    b.map(|b| b.acpl_avg).unwrap_or(100.0)
}

/// Population phase error-share norms (percent of key-moment errors) for the player's rating bucket.
pub fn phase_error_shares_for_rating(rating: i64) -> PhaseErrorShares {
    let key = bucket_key_for_rating(rating);
    let d = data();
    let b = d.buckets.get(&key).or_else(|| d.buckets.values().next());
    let Some(b) = b else {
        return PhaseErrorShares {
            opening: 16,
            middlegame: 28,
            endgame: 56,
        };
    };
    PhaseErrorShares {
        opening: b.phase_error_share_opening.round() as i64,
        middlegame: b.phase_error_share_middlegame.round() as i64,
        endgame: b.phase_error_share_endgame.round() as i64,
    }
}
