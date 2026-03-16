use anyhow::Result;
use futures::future::join_all;
use gtfs_realtime::FeedMessage;
use prost::Message;

use crate::types::Feed;

/// Fetch and decode a single GTFS-RT feed.
pub async fn fetch_feed(feed: &Feed) -> Result<FeedMessage> {
    let bytes = reqwest::get(feed.url()).await?.bytes().await?;
    let msg = FeedMessage::decode(bytes)?;
    Ok(msg)
}

/// Fetch and decode a GTFS-RT feed from an arbitrary URL.
pub async fn fetch_feed_url(url: &str) -> Result<FeedMessage> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    let msg = FeedMessage::decode(bytes)?;
    Ok(msg)
}

/// Fetch multiple feeds concurrently, returning all successful results.
pub async fn fetch_feeds(feeds: &[Feed]) -> Vec<(Feed, FeedMessage)> {
    let futures: Vec<_> = feeds
        .iter()
        .map(|f| {
            let feed = *f;
            async move {
                match fetch_feed(&feed).await {
                    Ok(msg) => Some((feed, msg)),
                    Err(e) => {
                        eprintln!("[cnctd_mta] Failed to fetch {:?}: {}", feed, e);
                        None
                    }
                }
            }
        })
        .collect();

    join_all(futures).await.into_iter().flatten().collect()
}
