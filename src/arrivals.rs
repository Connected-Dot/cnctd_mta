use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::feeds::fetch_feeds;
use crate::stations::{find_by_stop_id, Station};
use crate::types::Direction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arrival {
    /// Route letter/number (e.g., "F", "G", "A")
    pub route: String,
    /// Direction of travel
    pub direction: Direction,
    /// Human-readable direction label (e.g., "Manhattan" or "Coney Island")
    pub direction_label: String,
    /// Unix timestamp of predicted arrival
    pub arrival_time: i64,
    /// Minutes until arrival (computed at fetch time)
    pub minutes_away: f64,
    /// The trip ID from GTFS-RT
    pub trip_id: String,
}

/// Get upcoming arrivals at a station, sorted by arrival time.
///
/// `stop_id` is the base GTFS stop ID (e.g., "F14" for Carroll St).
/// Returns arrivals for both directions.
pub async fn get_arrivals(stop_id: &str) -> Result<Vec<Arrival>> {
    let station = find_by_stop_id(stop_id)
        .ok_or_else(|| anyhow::anyhow!("Unknown stop_id: {}", stop_id))?;

    get_arrivals_for_station(station).await
}

/// Get upcoming arrivals at a station using a Station reference.
pub async fn get_arrivals_for_station(station: &Station) -> Result<Vec<Arrival>> {
    let feeds_needed = station.feeds();
    if feeds_needed.is_empty() {
        return Ok(vec![]);
    }

    let (north_id, south_id) = station.directional_stop_ids();
    let now = Utc::now().timestamp();

    let feed_results = fetch_feeds(&feeds_needed).await;

    let mut arrivals = Vec::new();

    for (_feed, msg) in &feed_results {
        for entity in &msg.entity {
            let Some(trip_update) = &entity.trip_update else {
                continue;
            };

            let route = trip_update
                .trip
                .route_id
                .clone()
                .unwrap_or_default();

            let trip_id = trip_update
                .trip
                .trip_id
                .clone()
                .unwrap_or_default();

            for stu in &trip_update.stop_time_update {
                let Some(sid) = &stu.stop_id else { continue };

                let direction = if sid == &north_id {
                    Direction::North
                } else if sid == &south_id {
                    Direction::South
                } else {
                    continue;
                };

                let arrival_time = stu
                    .arrival
                    .as_ref()
                    .and_then(|a| a.time)
                    .or_else(|| stu.departure.as_ref().and_then(|d| d.time));

                let Some(arrival_time) = arrival_time else {
                    continue;
                };

                // Skip trains that have already passed
                if arrival_time < now {
                    continue;
                }

                let minutes_away = (arrival_time - now) as f64 / 60.0;

                let direction_label = match direction {
                    Direction::North => station.north_label.clone(),
                    Direction::South => station.south_label.clone(),
                };

                arrivals.push(Arrival {
                    route,
                    direction,
                    direction_label,
                    arrival_time,
                    minutes_away,
                    trip_id,
                });

                // Only take the first matching stop_time_update per trip
                break;
            }
        }
    }

    arrivals.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));
    Ok(arrivals)
}
