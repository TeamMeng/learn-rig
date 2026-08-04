use anyhow::Result;
use futures_util::StreamExt;
use rig::{
    agent::MultiTurnStreamItem,
    client::{AgentClientExt, Nothing},
    message::Message,
    providers::ollama,
    streaming::{StreamedAssistantContent, StreamingChat},
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("你是一个旅游顾问，帮助用户规划旅行")
        .build();

    let history: Vec<Message> = Vec::new();

    let mut stream = agent
        .stream_chat("我想去云南旅游，有什么推荐?", history.clone())
        .await;

    while let Some(chunk) = stream.next().await {
        if let Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(text))) =
            chunk
        {
            print!("{}", text.text)
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
