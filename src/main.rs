use miniflux_api::MinifluxApi;
use url::Url;
use reqwest::Client;
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please provide the url and filepath to the api token for the miniflux instance")
    }
    let api_token = fs::read_to_string(args[2].clone())?.replace("\n", "");
    let url = Url::parse(&args[1])?;
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
            let video_id = &entry.url[31..];
            let short_check_url = format!("https://www.youtube.com/shorts/{}", video_id);
            dbg!(short_check_url.clone());
            if !client.head(short_check_url).send().await?.status().is_redirection(){
                entries_to_update.push(entry.id)
            }
        }
    }

    if entries_to_update.len() != 0 {
        // Update all the entries
        mfx.update_entries_status(entries_to_update, miniflux_api::models::EntryStatus::Read, &client).await?;
    } else {
        println!("No entries to update");
    }

    Ok(())
}
