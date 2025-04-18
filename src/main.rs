use clap::Parser;
use regex::Regex;

/// Extract YouTube video ID and output GitHub README-style Markdown.
#[derive(Parser)]
#[command(name = "yt-md-gen")]
#[command(author = "Your Name <you@example.com>")]
#[command(version = "1.0")]
#[command(about = "Generate Markdown for embedding YouTube videos in GitHub README.md", long_about = None)]
struct Args {
    /// YouTube video URL (e.g., https://youtu.be/dQw4w9WgXcQ)
    url: String,
}

fn extract_video_id(url: &str) -> Option<String> {
    let re = Regex::new(r"(?:https?://(?:www\.)?youtube\.com/watch\?v=|https?://(?:www\.)?youtu\.be/)([a-zA-Z0-9_-]+)").unwrap();
    re.captures(url).map(|cap| cap[1].to_string())
}

fn main() {
    let args = Args::parse();

    match extract_video_id(&args.url) {
        Some(video_id) => {
            println!(
                "![Alt text](https://img.youtube.com/vi/{}/0.jpg)\n\n[Watch the video](https://www.youtube.com/watch?v={})",
                video_id, video_id
            );
        }
        None => {
            eprintln!("Invalid URL. Please provide a valid YouTube URL.");
        }
    }
}
