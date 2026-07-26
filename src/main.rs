use anyhow::Result;
use rig::{
    Embed,
    client::{EmbeddingsClient, Nothing},
    embeddings::EmbeddingsBuilder,
    providers::ollama,
};

#[allow(unused)]
#[derive(Embed, Clone)]
struct KnowledegItem {
    id: String,

    #[embed]
    content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let model = client.embedding_model(ollama::NOMIC_EMBED_TEXT);

    let documents = vec![
        KnowledegItem {
            id: "1".to_string(),
            content: "北京是中国的首都，有悠久的历史文化".to_string(),
        },
        KnowledegItem {
            id: "2".to_string(),
            content: "上海是中国最大的经济中心城市".to_string(),
        },
        KnowledegItem {
            id: "3".to_string(),
            content: "广州是中国南方的重要商贸城市".to_string(),
        },
    ];

    let embeddings = EmbeddingsBuilder::new(model)
        .documents(documents)?
        .build()
        .await?;

    println!("成功生成 {} 个嵌入", embeddings.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
