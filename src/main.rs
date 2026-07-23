use anyhow::Result;
use rig::{
    client::{CompletionClient, Nothing},
    completion::CompletionRequestBuilder,
    providers::ollama,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;
    let model = client.completion_model("qwen2.5:7b");

    let response = CompletionRequestBuilder::new(model, "写一首关于春天的诗")
        .preamble("你是一位擅长写古诗的诗人".to_string())
        .temperature(0.8)
        .max_tokens(200)
        .send()
        .await?;

    if let rig::completion::AssistantContent::Text(text) = response.choice.first() {
        println!("诗歌内容: {}\n", text);
    }

    println!("input token: {}", response.usage.input_tokens);
    println!("output token: {}", response.usage.output_tokens);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
