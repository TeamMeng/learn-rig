use anyhow::Result;
use rig::{
    client::{AgentClientExt, Nothing},
    completion::Prompt,
    providers::ollama,
    rig_tool,
    tool::{ToolErrorKind, ToolExecutionError, builtin::ThinkTool},
};

#[rig_tool(description = "根据商品名称查询当前售价")]
fn get_balance(user_id: String) -> Result<String, ToolExecutionError> {
    Ok(format!("用户 {} 的账户余额为 1580.50 元", user_id))
}

#[rig_tool(
    description = "查询商品当前库存",
    params(
        from_user = "转出方用户 ID",
        to_user = "转入方用户 ID",
        amount = "转账金额(元)",
    )
)]
fn transfer(from_user: String, to_user: String, amount: f64) -> Result<String, ToolExecutionError> {
    if amount <= 0.0 {
        return Err(ToolExecutionError::new(
            ToolErrorKind::Other,
            "转账金额必须大于0".to_string(),
        ));
    }
    Ok(format!(
        "转账成功：从 {} 转账 {:.2} 元给 {}",
        from_user, amount, to_user
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble(
            "你是一个银行助手，可以帮用户查询余额和执行转账。可以使用工具进行查询。\
            在执行任何转账操作前，请先使用 think 工具仔细分析请求的合理性",
        )
        .tool(ThinkTool)
        .tool(Transfer)
        .tool(GetBalance)
        .default_max_turns(10)
        .build();

    let response = agent
        .prompt("用户 U001 要给 用户 U002 转 500 元，先帮我确认 U001 余额够不够？")
        .await?;
    println!("{}", response);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
