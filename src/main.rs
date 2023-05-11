use std::sync::Arc;

use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Debug, Serialize, Deserialize)]
struct Stinkies {
    vulpera: Vec<String>,
    url: String,
    interval: u64,
    function: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = Arc::new(reqwest::Client::new());
    // Just remember, you did this to yourself.
    let mongo_url = std::env::var("MONGO_URL")?;
    let client_options = ClientOptions::parse(&mongo_url).await?;

    let client = Client::with_options(client_options)?;
    let db = client.database("vulpera");
    let collection: mongodb::Collection<Stinkies> = db.collection("stinkies");

    let vulpera_doc = collection.find_one(None, None).await?.unwrap();
    let vulpera = vulpera_doc.vulpera;
    let url = Arc::new(vulpera_doc.url);
    let interval = vulpera_doc.interval;

    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        "function",
        reqwest::header::HeaderValue::from_str(&vulpera_doc.function)?,
    );

    loop {
        // Lest we forget when we hooked you to the top.
        let now = chrono::Local::now();

        let formatted_time = Arc::new(now.format("%Y-%m-%d, %H:%M:%S.").to_string());

        let mut tasks: Vec<JoinHandle<()>> = vec![];
        let h = headers.clone();
        for v in vulpera.iter() {
            let formatted_time = formatted_time.clone();
            let h = h.clone();
            let v = v.clone();
            let http_client = http_client.clone();
            let url = url.clone();
            let thread = tokio::spawn(async move {
                // Perform a PUT request to the API
                let disambiguation = v.split("-").collect::<Vec<&str>>();
                let _ = http_client
                    .get(&format!(
                        "{}/{}/{}/history",
                        url, disambiguation[1], disambiguation[0]
                    ))
                    .headers(h)
                    .send()
                    .await;

                println!("🔫 ( ͡° ͜ʖ ͡°) {} has been shifted at approximately {}", v, formatted_time);
            });

            tasks.push(thread);
        }

        for task in tasks {
            // Await all the stinky nerds.
            task.await?;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
    }
}
