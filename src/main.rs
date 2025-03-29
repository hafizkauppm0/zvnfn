use std::error::Error;
use rfd::MessageDialog;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize)]
struct UnlockRequest {
    item_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

struct App {
    client: Client,
}

impl App {
    async fn unlock_items(&self, item_ids: Vec<String>) -> Result<(), Box<dyn Error>> {
        let request = UnlockRequest { item_ids };
        let response: UnlockResponse = self.client.post("https://api.fortnite.dev/unlock")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;
        
        if response.success {
            MessageDialog::new().set_title("Success").set_description(&response.message).show();
        } else {
            MessageDialog::new().set_title("Error").set_description(&response.message).show();
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let app = App { client };

    let item_ids = vec![
        "outfit1".to_string(),
        "emote1".to_string(),
        "pickaxe1".to_string(),
    ];

    app.unlock_items(item_ids).await?;
    Ok(())
}