use anyhow::Result;
use rig::{
    agent::AgentBuilder,
    client::{CompletionClient, Nothing},
    completion::Prompt,
    providers::ollama,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;
    let model = client.completion_model("qwen2.5:7b");

    let agent = AgentBuilder::new(model)
        .preamble(
            "你是一名专业的餐厅点餐助手，帮助顾客了解菜单并推荐菜品。请用友好、热情的语气回答。",
        )
        .context("今日菜单：红烧肉（38元）、清蒸鲈鱼（68元）、麻婆豆腐（22元）、蒜蓉炒青菜（18元）、番茄蛋花汤（12元）。")
        .context("特别提示：今日红烧肉是厨师长推荐，限量供应。所有菜品均可打包。")
        .temperature(0.4)
        .max_tokens(2048)
        .build();

    let res = agent.prompt("你们今天有什么推荐的菜嘛？").await?;

    println!("{}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
