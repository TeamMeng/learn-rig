use anyhow::Result;
use rig::{
    client::{AgentClientExt, Nothing},
    memory::InMemoryConversationMemory,
    providers::ollama,
};

#[tokio::main]
async fn main() -> Result<()> {
    let memory = InMemoryConversationMemory::new();

    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("你是一个友好的助手，请记住用户告诉你的消息。")
        .memory(memory)
        .build();

    agent
        .runner("我叫张伟 今年 21 岁")
        .conversation("thread-1")
        .run()
        .await?;

    let answer1 = agent
        .runner("我叫什么名字?")
        .conversation("thread-1")
        .run()
        .await?;

    println!("{:?}", answer1.content);

    let answer2 = agent
        .runner("我叫什么名字?")
        .conversation("thread-2")
        .run()
        .await?;

    println!("{:?}", answer2.content);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
