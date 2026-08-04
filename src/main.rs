use anyhow::Result;
use rig::{
    agent::stream_to_stdout,
    client::{AgentClientExt, Nothing},
    providers::ollama,
    streaming::StreamingPrompt,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("你是一个厨师助手，擅长介绍中国菜的做法。")
        .build();

    let mut stream = agent.stream_prompt("请介绍一下红烧肉的做法").await;
    stream_to_stdout(&mut stream).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
