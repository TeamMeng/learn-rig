use anyhow::Result;
use rig::{
    Embed,
    client::{EmbeddingsClient, Nothing},
    embeddings::{EmbeddingsBuilder, distance::VectorDistance},
    providers::ollama,
};

#[allow(unused)]
#[derive(Embed, Clone, Debug)]
struct Recipe {
    id: u32,
    name: String,

    #[embed]
    description: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let model = client.embedding_model(ollama::NOMIC_EMBED_TEXT);

    let recipes = vec![
        Recipe {
            id: 1,
            name: "红烧肉".to_string(),
            description: "猪五花肉加酱油冰糖慢炖，色泽红亮，肥而不腻".to_string(),
        },
        Recipe {
            id: 2,
            name: "清蒸鱼".to_string(),
            description: "新鲜鲈鱼蒸熟后淋上热油和酱油，鲜嫩爽口".to_string(),
        },
        Recipe {
            id: 3,
            name: "番茄炒蛋".to_string(),
            description: "鸡蛋和番茄翻炒，酸甜可口，家常快手菜".to_string(),
        },
        Recipe {
            id: 4,
            name: "麻婆豆腐".to_string(),
            description: "豆腐加牛肉末、豆瓣酱烹制，麻辣鲜香，下饭神器".to_string(),
        },
    ];

    let recipe_embeddings = EmbeddingsBuilder::new(model.clone())
        .documents(recipes)?
        .build()
        .await?;

    println!("已为 {} 个菜谱生成嵌入向量", recipe_embeddings.len());

    let query = "我想吃辣的菜".to_string();
    let query_embeddings = EmbeddingsBuilder::new(model)
        .documents(vec![query.clone()])?
        .build()
        .await?;

    let query_vec = &query_embeddings[0].1.first_ref();

    let mut scord: Vec<(f64, &str)> = recipe_embeddings
        .iter()
        .map(|(recipe, embeddings)| {
            (
                query_vec.cosine_similarity(embeddings.first_ref(), false),
                recipe.name.as_str(),
            )
        })
        .collect();

    scord.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    println!("query: {}", query);
    println!("similarity: ");
    for (rank, (score, name)) in scord.iter().enumerate() {
        println!(" {}. {} ({:4})", rank + 1, name, score);
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
