use anyhow::Result;
use rig::{
    agent::AgentBuilder,
    client::{CompletionClient, EmbeddingsClient, Nothing},
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
    providers::ollama,
    vector_store::in_memory_store::InMemoryVectorStore,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let embedding_model = client.embedding_model(ollama::NOMIC_EMBED_TEXT);

    let documents = vec![
        "极光系统：公司自研的智能排班系统，支持多门店、多岗位的自动化排班，可根据历史客流数据预测用工需求。",
        "星河平台：面向中小企业的一站式数字化运营平台，集成了进销存、财务、客户关系管理三大核心模块。",
        "蜂巢网络：公司的分布式边缘计算网络，在全国200个城市部署了边缘节点，平均延迟低于10毫秒。",
        "猎鹰算法：公司核心的推荐算法，基于用户行为序列建模，点击率比行业基准高出35%。",
    ];

    let mut vector_store = InMemoryVectorStore::default();

    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(documents)?
        .build()
        .await?;

    vector_store.add_documents(embeddings);

    let index = vector_store.index(embedding_model);

    let model = client.completion_model("qwen2.5:7b");

    let agent = AgentBuilder::new(model)
        .preamble(
            "你是公司的智能知识库助手，帮助员工了解公司内部产品和系统。
            你会收到一些相关的产品说明作为参考，请基于这些说明来回答问题。
            如果参考内容中没有相关信息，请如实告知。",
        )
        .dynamic_context(2, index)
        .temperature(0.3)
        .build();

    let res = agent.prompt("猎鹰算法是什么？效果怎么样？").await?;
    println!("answer: {}", res);

    let res = agent.prompt("极光系统支持哪些功能？").await?;
    println!("answer: {}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
