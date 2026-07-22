use anyhow::Result;
use rig::{
    client::{CompletionClient, Nothing},
    completion::Prompt,
    providers::ollama,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("你是一个友善的中文助手，回答要精简。")
        .build();

    let answer = agent.prompt("请介绍一下长城").await?;

    println!("{}", answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
