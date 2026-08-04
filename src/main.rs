use anyhow::Result;
use rig::{
    client::{AgentClientExt, Nothing},
    completion::Prompt,
    providers::ollama,
    rig_tool,
    tool::ToolExecutionError,
};

#[rig_tool(description = "查询城市的当前天气")]
async fn get_weather(city: String) -> Result<String, ToolExecutionError> {
    Ok(format!("{}: 晴天，气温 28 度", city))
}

#[rig_tool(description = "执行加法运算")]
fn add(a: i32, b: i32) -> Result<i32, ToolExecutionError> {
    Ok(a + b)
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("llama3.2")
        .preamble("你是一个助手，可以查询天气的执行计算")
        .tool(GetWeather)
        .tool(Add)
        .default_max_turns(10)
        .build();

    let response = agent.prompt("北京天气今天怎么样？").await?;
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
