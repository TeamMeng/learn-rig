use anyhow::Result;
use rig::{
    client::{CompletionClient, Nothing},
    completion::TypedPrompt,
    providers::ollama,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct PersonInfo {
    name: String,
    age: u32,
    city: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent("qwen2.5:7b")
        .preamble("从用户输入中提取人员信息。")
        .build();

    let answer: PersonInfo = agent.prompt_typed("我叫张伟，今年28岁，住在上海").await?;

    println!("{:?}", answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
