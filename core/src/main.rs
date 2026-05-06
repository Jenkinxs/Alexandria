use rustypipe::client::RustyPipe;
use rustypipe::model::VideoItem; 
use anyhow::Result;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let num_vids = 2;
    let search_term = "Rust programming tutorial";
    
    
    let video_ids = get_video_ids(num_vids, search_term).await?;

    for id in video_ids {
        println!("\nTranscript for: {}\n", id);
        run_ytx(&id);
    }

    Ok(())
}


fn run_ytx(id: &str) {
    
    Command::new("ytx")
        .arg(id)
        .status() 
        .expect("failed to run command");
}


async fn get_video_ids(num_vids: usize, search_term: &str) -> Result<Vec<String>> {
    let rp = RustyPipe::new();
    let mut id_list = Vec::new();

    println!("Searching for {} videos matching: '{}'...", num_vids, search_term);

    let mut search_results = rp.query()
        .search::<VideoItem, _>(search_term)
        .await?;

    search_results.items.extend_limit(&rp.query(), num_vids).await?;

    let videos: Vec<_> = search_results.items.items
        .into_iter()
        .take(num_vids)
        .collect();

    if videos.is_empty() {
        println!("No videos found.");
    } else {
        for (i, video) in videos.iter().enumerate() {
            println!("{}. {} (ID: {})", i + 1, video.name, video.id);
            id_list.push(video.id.clone());
        }
    }

    // Return the list wrapped in Ok
    Ok(id_list)
}
