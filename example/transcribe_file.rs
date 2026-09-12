//! Transcribe a local audio file with the Speechall API.
//!
//! ```sh
//! SPEECHALL_API_KEY=... cargo run --example transcribe-file -- \
//!   ./recording.wav openai.gpt-4o-mini-transcribe
//! ```

use std::{env, fs, process::ExitCode};

use speechall::ClientBuilder;

#[tokio::main]
async fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let mut arguments = env::args().skip(1);
    let Some(audio_path) = arguments.next() else {
        return usage();
    };
    let Some(model) = arguments.next() else {
        return usage();
    };
    if arguments.next().is_some() {
        return usage();
    }

    let api_key = env::var("SPEECHALL_API_KEY")
        .map_err(|_| "SPEECHALL_API_KEY must contain a Speechall API key")?;
    let audio = fs::read(&audio_path)?;

    let client = ClientBuilder::new().bearer_auth(api_key).build()?;
    let response = client
        .transcribe(
            &model,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            reqwest::Body::from(audio),
        )
        .await?;

    println!("{response:#?}");
    Ok(ExitCode::SUCCESS)
}

fn usage() -> Result<ExitCode, Box<dyn std::error::Error>> {
    eprintln!(
        "Usage: SPEECHALL_API_KEY=... cargo run --example transcribe-file -- \\\n+         <audio-path> <provider.model>\n\
         Example: SPEECHALL_API_KEY=... cargo run --example transcribe-file -- \\\n+         ./recording.wav openai.gpt-4o-mini-transcribe"
    );
    Ok(ExitCode::from(2))
}
