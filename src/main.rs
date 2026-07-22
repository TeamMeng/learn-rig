use anyhow::Result;
use rig::{
    client::{CompletionClient, Nothing},
    completion::{Chat, Message},
    providers::ollama,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("你是一个友善的中文助手，回答要精简。")
        .build();

    let mut history = vec![
        Message::user("你好，我叫小明"),
        Message::assistant("你好，小明，很高兴认识你。"),
        Message::user("我喜欢编程"),
        Message::assistant("太棒了，编程是一项很有用的技能。"),
    ];

    let answer = agent.chat("你还记得我叫什么名字嘛？", &mut history).await?;

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
