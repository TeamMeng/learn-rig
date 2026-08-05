use anyhow::Result;
use futures_util::StreamExt;
use rig::{
    agent::MultiTurnStreamItem,
    client::{AgentClientExt, Nothing},
    providers::ollama,
    streaming::{StreamedAssistantContent, StreamingPrompt, ToolCallDeltaContent},
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("你是一个查询天气的助手")
        .build();

    let mut stream = agent.stream_prompt("今天北京天气怎么样？").await;

    while let Some(result) = stream.next().await {
        match result {
            Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(text))) => {
                println!("{}", text.text);
            }
            Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::ToolCall {
                tool_call,
                ..
            })) => {
                println!(
                    "Tool Call: name{}, arguments: {}",
                    tool_call.function.name, tool_call.function.arguments
                )
            }
            Ok(MultiTurnStreamItem::StreamAssistantItem(
                StreamedAssistantContent::ToolCallDelta { content, .. },
            )) => match content {
                ToolCallDeltaContent::Name(name) => {
                    println!("Tool name: {}", name)
                }
                ToolCallDeltaContent::Delta(args) => {
                    println!("{}", args)
                }
            },
            _ => {}
        }
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
