//! Embedded rating benchmarks (`resources/benchmarks.json`).
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

#[derive(Debug, Deserialize)]
struct BucketRaw {
    label: String,
    pentagon: Pentagon,
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
    /// Sorted ascending by max_rating for lookup.
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

/// First rule with `rating <= max` wins (`rules` sorted by `max`).
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

pub fn pentagon_and_label(bucket_key: &str) -> Option<(Pentagon, String)> {
    let d = data();
    d.buckets.get(bucket_key).map(|b| (b.pentagon.clone(), b.label.clone()))
}
