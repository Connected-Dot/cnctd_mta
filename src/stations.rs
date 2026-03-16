use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::LazyLock;

use crate::types::{route_to_feed, Feed};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub stop_id: String,
    pub name: String,
    pub borough: String,
    pub daytime_routes: String,
    pub complex_id: String,
    pub north_label: String,
    pub south_label: String,
    pub lat: f64,
    pub lon: f64,
}

impl Station {
    /// Returns the individual route letters/numbers as a Vec.
    pub fn routes(&self) -> Vec<String> {
        self.daytime_routes
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    /// Returns the set of GTFS-RT feeds needed to get data for this station.
    pub fn feeds(&self) -> Vec<Feed> {
        let mut seen = HashSet::new();
        self.routes()
            .iter()
            .filter_map(|r| route_to_feed(r))
            .filter(|f| seen.insert(*f))
            .collect()
    }

    /// Returns the directional stop_ids used in GTFS-RT feeds.
    pub fn directional_stop_ids(&self) -> (String, String) {
        (format!("{}N", self.stop_id), format!("{}S", self.stop_id))
    }
}

static STATIONS: LazyLock<Vec<Station>> = LazyLock::new(|| {
    let json = include_str!("stations.json");
    serde_json::from_str(json).expect("Failed to parse embedded stations data")
});

/// Get all stations.
pub fn all_stations() -> &'static [Station] {
    &STATIONS
}

/// Find a station by its GTFS stop_id (e.g., "F14" for Carroll St).
pub fn find_by_stop_id(stop_id: &str) -> Option<&'static Station> {
    STATIONS
        .iter()
        .find(|s| s.stop_id.eq_ignore_ascii_case(stop_id))
}

/// Find stations in the same complex (transfer station).
pub fn find_complex(complex_id: &str) -> Vec<&'static Station> {
    STATIONS
        .iter()
        .filter(|s| s.complex_id == complex_id)
        .collect()
}

/// Search stations by name (case-insensitive substring match).
pub fn search(query: &str) -> Vec<&'static Station> {
    let q = query.to_lowercase();
    STATIONS
        .iter()
        .filter(|s| s.name.to_lowercase().contains(&q))
        .collect()
}

/// Find all stations served by a specific route.
pub fn find_by_route(route: &str) -> Vec<&'static Station> {
    let r = route.to_uppercase();
    STATIONS
        .iter()
        .filter(|s| s.routes().iter().any(|sr| sr.to_uppercase() == r))
        .collect()
}

/// Strip the direction suffix (N/S) from a GTFS-RT stop_id to get the base stop_id.
pub fn base_stop_id(directional_stop_id: &str) -> &str {
    directional_stop_id
        .strip_suffix('N')
        .or_else(|| directional_stop_id.strip_suffix('S'))
        .unwrap_or(directional_stop_id)
}
