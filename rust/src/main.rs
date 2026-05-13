use rustypipe::client::RustyPipe;
use rustypipe::model::VideoItem; 
use anyhow::Result;
use std::process::Command;
use rusqlite::{Connection, Result};
    



#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::open("alexandria.db")?;
    create_table(&conn)?;
    
    // Example usage (you'll need to call these where appropriate)
    // insert_row(&conn, 1, "Video Title", "positive", "transcript text", "https://...")?;
    // query(&conn)?;


    let num_vids = 2;
    let search_term = "Philosophy course";
    
    
    let video_ids = get_video_ids(num_vids, search_term).await?;

    for id in video_ids {
        println!("\nTranscript for: {}\n", id);
        //insert_row(&conn, 1, "Video Title", "positive", "transcript text", "https://...")?;
        run_ytx(&id);
    }

    Ok(())
}


fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS video_info (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            sentiment TEXT NOT NULL,
            transcript TEXT NOT NULL,
            url TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn insert_row(conn: &Connection, id: i64, title: &str, sentiment: &str, transcript: &str, url: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO video_info (id, title, sentiment, transcript, url) VALUES (?1, ?2, ?3, ?4, ?5)",
        [id, title, sentiment, transcript, url],
    )?;
    Ok(())
}


fn query(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, title, transcript FROM video_info")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    
    for row in rows {
        let (id, title, transcript) = row?;
        println!("ID: {}, Title: {}, Transcript: {}", id, title, transcript);
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
