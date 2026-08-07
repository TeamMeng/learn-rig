use anyhow::Result;
use rig::{
    client::{AgentClientExt, Nothing},
    memory::{
        CompactingMemory, InMemoryConversationMemory, SlidingWindowMemory, TemplateCompactor,
    },
    providers::ollama,
};

#[tokio::main]
async fn main() -> Result<()> {
    let memory = CompactingMemory::new(
        InMemoryConversationMemory::new(),
        SlidingWindowMemory::last_messages(10),
        TemplateCompactor::with_header("早期对话摘要").with_max_bytes(2 * 1024),
    );

    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("你是一个长期陪伴用户的生活助手，请记住用户的偏好和习惯。")
        .memory(memory)
        .build();

    let infos = vec![
        "我喜欢喝绿茶，不喜欢咖啡。",
        "我住在北京，上班地点在中关村。",
        "我有一只猫，名字叫橘子。",
        "我每天早上7点起床，晚上11点睡觉。",
        "我最近在学习Rust编程语言。",
        "我喜欢周末去爬山。",
        "我不吃辣，对海鲜过敏。",
        "我的生日是3月15日。",
        "我喜欢听古典音乐，特别是肖邦的作品。",
        "我最近在读《三体》这本书。",
        "我的工作是软件工程师，主要做后端开发。",
        "我有一个弟弟，比我小三岁。",
    ];

    for info in infos {
        agent
            .runner(info)
            .conversation("user_longterm")
            .run()
            .await?;
    }

    let answer = agent
        .runner("我喜欢喝什么饮料？我有宠物嘛？")
        .conversation("user_longterm")
        .run()
        .await?;

    println!("{:#?}", answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
