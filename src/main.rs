use chrono::Utc;
use clap::{Command, arg};
use firestore::*;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MicroblogStruct {
    id: String,
    content: String,
    time: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get the content argument from command line
    let matches = Command::new("microblog")
        .version("1.0")
        .author("Akshith Garapati")
        .about("Create a new microblog post")
        .arg(arg!([content] "The content of the microblog post").required(true))
        .get_matches();

    // Get the content argument
    let content = matches.get_one::<String>("content").unwrap();

    // Generate random ID and get current timestamp
    let id = Uuid::new_v4().to_string();
    let timestamp = Utc::now().to_rfc3339();

    // Fetch environment variables (these should be in your shell environment, e.g., .zshrc)
    let project_id = env::var("PROJECT_ID").map_err(|e| format!("PROJECT_ID not found: {}", e))?;
    let google_credentials = env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .map_err(|e| format!("GOOGLE_APPLICATION_CREDENTIALS not found: {}", e))?;

    // Set the GOOGLE_APPLICATION_CREDENTIALS environment variable inside an unsafe block
    unsafe {
        std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", google_credentials);
    }

    // Firestore client initialization
    let db = FirestoreDb::new(&project_id).await?;

    const COLLECTION_NAME: &'static str = "microblog";

    // Create new microblog post
    let microblog_struct = MicroblogStruct {
        id,
        content: content.to_string(),
        time: timestamp,
    };

    // Insert document into Firestore
    let object_returned: MicroblogStruct = db
        .fluent()
        .insert()
        .into(COLLECTION_NAME)
        .document_id(&microblog_struct.id)
        .object(&microblog_struct)
        .execute()
        .await?;

    println!("Inserted: {:?}", object_returned);

    Ok(())
}
