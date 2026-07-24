use anyhow::Result;
use rig::{
    agent::AgentBuilder,
    client::{CompletionClient, Nothing},
    completion::Chat,
    providers::ollama,
};
use std::io::{BufRead, stdin};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;
    let model = client.completion_model("qwen2.5:7b");

    let agent = AgentBuilder::new(model)
        .preamble("你是一名电商平台的客服，负责解答用户关于订单、退款、物流的问题。")
        .context("退款政策：收到商品7天内可申请无理由退款，质量问题30天内可退换。")
        .context("物流说明：普通快递3-5天，加急快递1-2天，偏远地区可能延迟。")
        .temperature(0.4)
        .max_tokens(2048)
        .build();

    let mut history = Vec::new();
    let stdin = stdin();

    println!("客服已上线，请输入您的问题（输入 exit 退出）：");

    for line in stdin.lock().lines() {
        let input = line?;
        if input.trim() == "exit" {
            break;
        }

        let res = agent.chat(input, &mut history).await?;
        println!("{}", res);
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
