use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::feeds::fetch_feed_url;
use crate::types::ALERTS_URL;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivePeriod {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAlert {
    pub alert_id: String,
    pub header: String,
    pub description: String,
    pub affected_routes: Vec<String>,
    pub affected_stops: Vec<String>,
    pub active_periods: Vec<ActivePeriod>,
}

/// Fetch all current subway service alerts.
pub async fn get_alerts() -> Result<Vec<ServiceAlert>> {
    let msg = fetch_feed_url(ALERTS_URL).await?;
    let mut alerts = Vec::new();

    for entity in &msg.entity {
        let Some(alert) = &entity.alert else {
            continue;
        };

        let header = alert
            .header_text
            .as_ref()
            .and_then(|t| t.translation.first())
            .map(|t| t.text.clone())
            .unwrap_or_default();

        let description = alert
            .description_text
            .as_ref()
            .and_then(|t| t.translation.first())
            .map(|t| t.text.clone())
            .unwrap_or_default();

        let mut affected_routes = Vec::new();
        let mut affected_stops = Vec::new();

        for ie in &alert.informed_entity {
            if let Some(route) = &ie.route_id {
                if !affected_routes.contains(route) {
                    affected_routes.push(route.clone());
                }
            }
            if let Some(stop) = &ie.stop_id {
                if !affected_stops.contains(stop) {
                    affected_stops.push(stop.clone());
                }
            }
        }

        let active_periods = alert
            .active_period
            .iter()
            .map(|p| ActivePeriod {
                start: p.start.map(|v| v as i64),
                end: p.end.map(|v| v as i64),
            })
            .collect();

        alerts.push(ServiceAlert {
            alert_id: entity.id.clone(),
            header,
            description,
            affected_routes,
            affected_stops,
            active_periods,
        });
    }

    Ok(alerts)
}

/// Fetch service alerts filtered to specific routes.
pub async fn get_alerts_for_routes(routes: &[&str]) -> Result<Vec<ServiceAlert>> {
    let all = get_alerts().await?;
    let route_set: Vec<String> = routes.iter().map(|r| r.to_uppercase()).collect();

    Ok(all
        .into_iter()
        .filter(|a| {
            a.affected_routes
                .iter()
                .any(|r| route_set.contains(&r.to_uppercase()))
        })
        .collect())
}
