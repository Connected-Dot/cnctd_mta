use cnctd_mta::{get_arrivals, get_alerts_for_routes, search};

#[tokio::main]
async fn main() {
    // Find Carroll St
    let results = search("Carroll");
    for s in &results {
        println!(
            "Station: {} ({}), stop_id={}, routes={}",
            s.name, s.borough, s.stop_id, s.daytime_routes
        );
    }

    println!("\n--- Arrivals at Carroll St (F14) ---");
    match get_arrivals("F14").await {
        Ok(arrivals) => {
            for a in arrivals.iter().take(10) {
                println!(
                    "  {} train {} → {} in {:.1} min",
                    a.route,
                    a.direction_label,
                    if a.direction == cnctd_mta::Direction::North {
                        "N"
                    } else {
                        "S"
                    },
                    a.minutes_away
                );
            }
            if arrivals.is_empty() {
                println!("  (no upcoming arrivals)");
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\n--- F/G Service Alerts ---");
    match get_alerts_for_routes(&["F", "G"]).await {
        Ok(alerts) => {
            for a in &alerts {
                println!("  [{}] {}", a.alert_id, a.header);
                if !a.description.is_empty() {
                    let desc = if a.description.len() > 120 {
                        format!("{}...", &a.description[..120])
                    } else {
                        a.description.clone()
                    };
                    println!("    {}", desc);
                }
            }
            if alerts.is_empty() {
                println!("  (no active alerts)");
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
}
