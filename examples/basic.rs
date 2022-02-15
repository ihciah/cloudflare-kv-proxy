use std::time::Duration;

use cloudflare_kv_proxy::Client;
use serde::{Deserialize, Serialize};

const ENDPOINT: &str = "https://YOUR-WORKER.workers.dev";
const TOKEN: &str = "YOUR-TOKEN";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = Client::new(ENDPOINT, TOKEN, 10, Duration::from_secs(120))
        .expect("unable to build kv proxy client");

    // String
    let key: &str = "test_string";
    println!("Put {key}: {:?}", client.put(key, "balabala").await);
    println!("Get {key}: {:?}", client.get::<String>(key).await);
    println!("Delete {key}: {:?}", client.delete(key).await);
    println!("Get {key}: {:?}", client.get::<String>(key).await);

    // Struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Demo {
        name: String,
        age: u8,
    }
    let data = Demo {
        name: "red".to_string(),
        age: 21,
    };
    let key: &str = "test_struct";
    println!("Put {key}: {:?}", client.put(key, &data).await);
    println!("Get {key}: {:?}", client.get::<Demo>(key).await);
    println!("Delete {key}: {:?}", client.delete(key).await);
    println!("Get {key}: {:?}", client.get::<Demo>(key).await);
}
