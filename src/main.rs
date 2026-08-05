use anyhow::Result;
use futures_util::StreamExt;
use rig::{
    agent::MultiTurnStreamItem,
    client::{AgentClientExt, Nothing},
    providers::ollama,
    streaming::{StreamedAssistantContent, StreamingPrompt},
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("你是一个写作助手")
        .build();

    let mut stream = agent.stream_prompt("写一篇关于熊猫的长文章").await;

    let mut count = 0;
    while let Some(chunk) = stream.next().await {
        if let Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(text))) =
            chunk
        {
            println!("{}", text.text);
            count += 1;

            if count >= 20 {
                break; // stream 被 drop,底层请求随之取消
            }
        }
    }

    println!("It has {} chunks", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
