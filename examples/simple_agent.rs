use std::env;

use anyhow::Result;
use rig::{AgentBuilder, client::CompletionClient, completion::Prompt, providers::deepseek};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = env::var("DEEPSEEK_API_KEY")?;

    let model = deepseek::Client::new(api_key)?.completion_model(deepseek::DEEPSEEK_V4_FLASH);

    let agent = AgentBuilder::new(model)
        .preamble("You are a comedian here to entertain the user using humour and jokes.")
        .build();

    let response = agent.prompt("Hello").await?;

    println!("{}", response);

    Ok(())
}
