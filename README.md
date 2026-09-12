# Speechall Rust SDK

Rust client and models for the [Speechall API](https://docs.speechall.com),
generated from the shared Speechall OpenAPI document.

```toml
[dependencies]
speechall = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust,no_run
use speechall::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new()
        .bearer_auth(std::env::var("SPEECHALL_API_KEY")?)
        .build()?;

    let response = client.list_speech_to_text_models().await?;
    println!("{response:?}");
    Ok(())
}
```

The generated API is available through `speechall::client`, while schema models
are available through `speechall::models`. The client defaults to
`https://api.speechall.com/v1`, disables redirects, and supports bearer
authentication with `ClientBuilder::bearer_auth`.

## Try a local audio file

The included executable transcribes a small local file and prints the typed
response. Supply a model identifier accepted by your Speechall account:

```sh
SPEECHALL_API_KEY=... cargo run --example transcribe-file -- \
  ./recording.wav openai.gpt-4o-mini-transcribe
```

The example reads the audio file into memory, so use it for small test files.
For large uploads, pass a streaming `reqwest::Body` to `Client::transcribe`.

## Regenerating

Install [`oapi-to-rust`](https://github.com/atacan/rust-openapi-generator)
and run:

```sh
brew install atacan/tap/oapi-to-rust
./regenerate.sh
```

`regenerate.sh` reads `../../Speechall-Repositories/speechall-openapi/openapi.yaml`,
the same source document used by the other SDKs. It intentionally generates
only `models.rs`, `views.rs`, and `client.rs`; no server surface is generated.
Set `OAPI_TO_RUST` to use a different generator binary, for example a local
development build.

The generator's `openapi-support` runtime crate is currently consumed from its
pinned upstream Git revision; Cargo.lock records the exact resolved dependency
graph.
