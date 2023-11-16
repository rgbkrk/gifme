use clap::Parser;

use base64;
use base64::{engine::general_purpose, Engine as _};
use reqwest;
use serde_json::Value;
use std::error::Error;

use atty::Stream;

#[macro_use]
extern crate query_params;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Rating for the GIF (e.g., g, pg, r)
    #[arg(long)]
    rating: Option<String>,

    /// Tags to filter GIFs
    #[arg(trailing_var_arg = true)]
    tags: Vec<String>,
}

#[derive(QueryParams)]
#[allow(non_snake_case)]
struct RandomGIFQuery {
    apiKey: String,
    rating: Option<String>,
    tag: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let tag = if args.tags.is_empty() {
        // The user ran `gifme` in the terminal with no pipes
        if atty::is(Stream::Stdin) {
            None
        // Getting piped input
        } else {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;
            Some(buffer.trim().to_string())
        }
    } else {
        Some(args.tags.join(","))
    };

    let query = RandomGIFQuery {
        apiKey: std::env::var("GIPHY_API_KEY")
            .unwrap_or_else(|_| "0UTRbFtkMxAplrohufYco5IY74U8hOes".to_string()),
        rating: args.rating,
        tag: tag,
    };

    let query_params = query.to_query_params();

    let url = format!("https://api.giphy.com/v1/gifs/random{}", query_params);

    let res = reqwest::get(url).await?.text().await?;
    let json: Value = serde_json::from_str(&res)?;

    let gif_url = json["data"]["images"]["downsized"]["url"]
        .as_str()
        .ok_or("No URL found")?;

    let gif_response = reqwest::get(gif_url).await?;
    let gif_bytes = gif_response.bytes().await?;

    let base64_gif = general_purpose::STANDARD_NO_PAD.encode(&gif_bytes);

    // iTerm2 format string let's go
    println!("\x1B]1337;File=;inline=1:{}\x07", base64_gif);

    Ok(())
}
