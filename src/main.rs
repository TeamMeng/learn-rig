use anyhow::Result;
use rig::{
    client::{CompletionClient, Nothing},
    completion::{CompletionRequestBuilder, Message},
    providers::ollama,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;
    let model = client.completion_model("qwen2.5:7b");

    let history = vec![
        Message::user("你好，我想了解一下Rust语音"),
        Message::assistant("你好！Rust 是一门注重安全性和性能的系统编程语音"),
    ];

    let response = CompletionRequestBuilder::new(model, "Rust 和 C++相比，主要优势是什么？")
        .preamble("你是一个 Rust 编程专家，回答要精简专业".to_string())
        .messages(history)
        .temperature(0.3)
        .max_tokens(500)
        .send()
        .await?;

    for content in response.choice.iter() {
        match content {
            rig::completion::AssistantContent::Text(text) => {
                println!("assistant answer: {}", text)
            }
            rig::completion::AssistantContent::ToolCall(tc) => {
                println!("tool call: {}", tc.function.name)
            }
            _ => {}
        }
    }

    if response.usage.has_values() {
        println!("\ninput: {}", response.usage.input_tokens);
        println!("output: {}", response.usage.output_tokens);
        println!("all: {}", response.usage.total_tokens);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
