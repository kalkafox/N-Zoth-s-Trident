use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

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

    loop {
        let vulpera = &vulpera_doc.vulpera;
        for v in vulpera {
            // Perform a PUT request to the API
            let disambiguation = v.split("-").collect::<Vec<&str>>();
            let res = http_client
                .put(format!("{}/{}/{}/battlenet", vulpera_doc.url, disambiguation[1], disambiguation[0]))
                .json(&v)
                .send()
                .await?;

            println!("{:?}", res);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}
