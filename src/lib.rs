pub mod api {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct UnlockRequest {
        pub item_ids: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UnlockResponse {
        pub success: bool,
        pub message: String,
    }
}