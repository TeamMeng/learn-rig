use anyhow::Result;
use chrono::Utc;
use rig::{
    client::{AgentClientExt, Nothing},
    completion::Prompt,
    providers::ollama,
    rig_tool,
    tool::{ToolErrorKind, ToolExecutionError},
};

#[rig_tool(
    description = "根据商品名称查询当前售价",
    params(product_name = "要查询价格的商品名称")
)]
async fn get_price(product_name: String) -> Result<String, ToolExecutionError> {
    let price = match product_name.as_str() {
        "手机" => "3999元",
        "耳机" => "299元",
        "充电器" => "89元",
        _ => "暂无价格消息",
    };
    Ok(format!("{}的售价为 {}", product_name, price))
}

#[rig_tool(
    description = "查询商品当前库存",
    params(product_name = "要查询库存的商品名称")
)]
fn check_stock(product_name: String) -> Result<String, ToolExecutionError> {
    let stock = match product_name.as_str() {
        "手机" => 128,
        "耳机" => 500,
        "充电器" => 1200,
        _ => 0,
    };
    if stock == 0 {
        Ok(format!("{}目前暂无库存", product_name))
    } else {
        Ok(format!("{}当前库存 {} 件", product_name, stock))
    }
}

#[rig_tool(
    description = "为用户提交购买订单",
    params(
        product_name = "要购买的商品名称",
        quantity = "购买数量",
        address = "收货地址",
    )
)]
fn place_order(
    product_name: String,
    quantity: i32,
    address: String,
) -> Result<String, ToolExecutionError> {
    if quantity <= 0 {
        return Err(ToolExecutionError::new(
            ToolErrorKind::Other,
            "购买数量必须大于0",
        ));
    }
    Ok(format!(
        "订单提交成功!商品: {}，数量: {}，收货地址: {}，订单号: ORD-{}",
        product_name,
        quantity,
        address,
        Utc::now()
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("你是一个专业的电商客服助手。本店商品只有三种：手机、耳机、充电器。\n规则：\n1. 用户提到\"手机\"时，直接以\"手机\"作为商品名称调用 get_price 或 check_stock 查询，不要反问具体型号。\n2. 当用户已提供商品、数量、收货地址时，直接调用 place_order 提交订单，不要要求用户再次确认。\n3. 根据工具返回的结果，用简洁友好的中文回复用户。")
        .tool(GetPrice)
        .tool(CheckStock)
        .tool(PlaceOrder)
        .default_max_turns(10)
        .build();

    let response = agent.prompt("手机多少钱？").await?;
    println!("{}", response);

    let response = agent
        .prompt("我想买2部手机，送到北京朝阳区建设路1号")
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
