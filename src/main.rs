use anyhow::Result;
use rig::{
    Embed,
    client::{EmbeddingsClient, Nothing},
    embeddings::EmbeddingsBuilder,
    providers::ollama,
};

#[allow(unused)]
#[derive(Embed, Clone)]
struct CompanyDoc {
    id: String,
    department: String,

    #[embed]
    title: String,

    #[embed]
    content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let model = client.embedding_model(ollama::NOMIC_EMBED_TEXT);

    let documents = vec![
        CompanyDoc {
            id: "hr-001".to_string(),
            department: "人力资源".to_string(),
            title: "员工年假申请流程".to_string(),
            content: "员工每年享有10天带薪年假，需提前3天通过OA系统申请".to_string(),
        },
        CompanyDoc {
            id: "fin-001".to_string(),
            department: "财务".to_string(),
            title: "报销流程说明".to_string(),
            content: "差旅肥报销需在出差结束后7个工作日内提交，附原始发票".to_string(),
        },
        CompanyDoc {
            id: "it-001".to_string(),
            department: "消息技术".to_string(),
            title: "VPN 使用说明".to_string(),
            content: "远程办公需使用公司VPN，下载客户端后输入工号和密码登陆".to_string(),
        },
    ];

    let embeddings = EmbeddingsBuilder::new(model)
        .documents(documents)?
        .build()
        .await?;

    for emb in &embeddings {
        println!(
            "文档 [{}] <{}> -> 向量维度：{}",
            emb.0.id,
            emb.0.title,
            emb.1.len()
        )
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
