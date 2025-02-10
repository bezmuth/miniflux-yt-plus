use miniflux_api::MinifluxApi;
use url::Url;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = "";
    let url = Url::parse("http://localhost:10002/")?;
    let client = Client::new();

    let mfx = MinifluxApi::new_from_token(&url, api_token.to_string());

    // get all the feeds from miniflux
    let feeds = mfx.get_feeds(&client).await?;
    // filter to only feeds from youtube
    let yt_feeds = feeds.into_iter()
                        .filter(|feed| feed.site_url.starts_with("https://www.youtube.com/channel/"))
                        .collect::<Vec<miniflux_api::models::Feed>>();

    // a suprise tool that will help us later
    let mut entries_to_update = Vec::new();
    for feed in yt_feeds{
        // get all the unread feeds
        for entry in mfx.get_feed_entries(feed.id, Some(miniflux_api::models::EntryStatus::Unread), None, None, None, None, None, None, None, None, None, &client ).await? {
            // check if its a short
            if entry.reading_time == 1 {
                // add the entry id to our update list
                entries_to_update.push(entry.id)
            }
        }
    }

    // Update all the entries
    mfx.update_entries_status(entries_to_update, miniflux_api::models::EntryStatus::Read, &client).await?;

    Ok(())
}
