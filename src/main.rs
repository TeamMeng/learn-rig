use anyhow::Result;
use rig::{
    memory::{ConversationMemory, InMemoryConversationMemory, IntoFilter, SlidingWindowMemory},
    message::Message,
};

#[tokio::main]
async fn main() -> Result<()> {
    // let client = ollama::Client::new(Nothing)?;
    //
    // let agent = client
    //     .agent("qwen2.5:7b")
    //     .preamble("你是一个的客服助手")
    //     .memory(memory)
    //     .build();

    let memory = InMemoryConversationMemory::new()
        .with_filter(SlidingWindowMemory::last_messages(20).into_filter());

    for i in 0..25 {
        memory
            .append("verify-window", vec![Message::user(format!("消息 {i}"))])
            .await?;
    }

    let visible = memory.load("verify-window").await?;
    assert_eq!(visible.len(), 20);
    println!("窗口加载到 {} 条消息", visible.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
