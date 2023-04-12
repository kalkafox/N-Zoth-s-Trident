use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Debug, Serialize, Deserialize)]
struct Stinkies {
    vulpera: Vec<String>,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = reqwest::Client::new();
    // Just remember, you did this to yourself.
    let mongo_url = std::env::var("MONGO_URL")?;
    let client_options = ClientOptions::parse(&mongo_url).await?;

    let client = Client::with_options(client_options)?;
    let db = client.database("vulpera");
    let collection: mongodb::Collection<Stinkies> = db.collection("stinkies");

    let vulpera_doc = collection.find_one(None, None).await?.unwrap();
    let vulpera = &vulpera_doc.vulpera;
    let url = &vulpera_doc.url;

    loop {
        let mut tasks: Vec<JoinHandle<()>> = vec![];
        for v in vulpera {
            let v = v.clone();
            let http_client = http_client.clone();
            let url = url.clone();
            let thread = tokio::spawn(async move {
                // Perform a PUT request to the API
                let disambiguation = v.split("-").collect::<Vec<&str>>();
                let _ = http_client
                    .put(&format!(
                        "{}/{}/{}/battlenet",
                        url, disambiguation[1], disambiguation[0]
                    ))
                    .send()
                    .await;

                println!("🔫 ( ͡° ͜ʖ ͡°) {} has been shifted.", v);
            });

            tasks.push(thread);
        }

        for task in tasks {
            // Await all the stinky nerds.
            task.await?;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}
