# Cloudflare Worker KV Proxy
[![Crates.io][crates-badge]][crates-url]
[![MIT/Apache-2 licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]
This project is implemented to use Cloudflare KV in non-worker environment.

## Setup KV and Proxy
1. Copy whole content of `worker/kv_proxy.js` to create a new Cloudflare Worker using your browser, and click `Save and Deploy`.
2. Create a new KV namespace.
3. Go to your worker(the worker you just created) settings, and:
    1. Add KV binding.
    2. Add environment variable `KEY`, the value will be your access token(please make it long enough).

## Usage
```rust
#[derive(Serialize, Deserialize, Debug)]
struct Demo {
    name: String,
    age: u8,
}
let data = Demo {
    name: "red".to_string(),
    age: 21,
};

let client = Client::new("https://your-proxy.workers.dev", "YOUR-TOKEN").unwrap();
println!("Put string: {:?}", client.put("test_key", "balabala").await);
println!("Get string: {:?}", client.get::<String>("test_key").await);

println!("Put struct: {:?}", client.put("test_key2", &data).await);
println!("Get struct: {:?}", client.get::<Demo>("test_key2").await);
```

## Cache
To avoid unnecessary requests to Cloudflare, the proxy caches the response.

By default the caching is enabled. You can set `default-features = false` in `Cargo.toml`.

[crates-badge]: https://img.shields.io/crates/v/cloudflare-kv-proxy.svg
[crates-url]: https://crates.io/crates/cloudflare-kv-proxy
[license-badge]: https://img.shields.io/crates/l/cloudflare-kv-proxy.svg
[license-url]: LICENSE-MIT
[actions-badge]: https://github.com/ihciah/cloudflare-kv-proxy/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/ihciah/cloudflare-kv-proxy/actions