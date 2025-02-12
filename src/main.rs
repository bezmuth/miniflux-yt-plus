use clap::Parser;
use miniflux_api::MinifluxApi;
use reqwest::Client;
use std::{fs, path::PathBuf};
use url::Url;

#[derive(Parser, Debug)]
#[command(version, about = "Remove youtube shorts and livestreams from miniflux", long_about = None, max_term_width=120)]
struct Args {
    /// Miniflux instance url
    url: Url,
    /// Path to file containing the miniflux api token
    tokenfile: PathBuf,
    /// Remove shorts
    #[arg(long, short = 's', required_unless_present = "remove_livestreams")]
    remove_shorts: bool,
    /// Remove livestreams
    #[arg(long, short = 'l', required_unless_present = "remove_shorts")]
    remove_livestreams: bool,
}

async fn check_for_livestream(url: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;
    // Only livestreams have "watching now" in the description
    if response.contains("watching now") {
        return Ok(true);
    }
    Ok(false)
}

async fn check_for_short(url: &str, client: &Client) -> Result<bool, Box<dyn std::error::Error>> {
    let video_id = &url[32..];
    let short_check_url = format!("https://www.youtube.com/shorts/{video_id}");
    // make a request to the url, if this video isn't a short it will redirect
    Ok(!client
        .head(short_check_url)
        .send()
        .await?
        .status()
        .is_redirection())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        // disable redirects so we can check if the video is a short
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let args = Args::parse();

    let api_token = fs::read_to_string(args.tokenfile)?.replace('\n', "");
    let mfx = MinifluxApi::new_from_token(&args.url, api_token.to_string());

    // get all the feeds from miniflux
    let feeds = mfx.get_feeds(&client).await?;
    // filter to only feeds from youtube
    let yt_feeds = feeds
        .into_iter()
        .filter(|feed| {
            feed.site_url
                .starts_with("https://www.youtube.com/channel/")
        })
        .collect::<Vec<miniflux_api::models::Feed>>();

    // suprise tools that will help us later
    let mut entries_to_update = Vec::new();
    for feed in yt_feeds {
        // get all the unread feeds
        for entry in mfx
            .get_feed_entries(
                feed.id,
                Some(miniflux_api::models::EntryStatus::Unread),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                &client,
            )
            .await?
        {
            if args.remove_shorts && check_for_short(&entry.url, &client).await? {
                entries_to_update.push(entry.id);
            }
            if args.remove_livestreams && check_for_livestream(&entry.url).await? {
                entries_to_update.push(entry.id);
            }
        }
    }

    if entries_to_update.is_empty() {
        // Update all the entries
        mfx.update_entries_status(
            entries_to_update,
            miniflux_api::models::EntryStatus::Read,
            &client,
        )
        .await?;
    } else {
        println!("No entries to update");
    }

    Ok(())
}
