use anyhow::Result;
use rig::{
    agent::stream_to_stdout,
    client::{AgentClientExt, EmbeddingsClient, Nothing, ProviderClient},
    embeddings::EmbeddingsBuilder,
    memory::InMemoryConversationMemory,
    providers::{ollama, openai},
    streaming::StreamingPrompt,
    vector_store::in_memory_store::InMemoryVectorStore,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let openai = openai::Client::from_env()?;
    // 网关不支持 embedding，本地 Ollama 提供向量化
    let ollama = ollama::Client::new(Nothing)?;

    let knowledge_base = [
        "华星Pro手机：屏幕6.7英寸2K，晓龙8Gen3，12GB内存，256GB存储，5000mAh电池，100W快充，约45分钟充满。",
        "128GB存储，4500mAh电池，33W快充，约75分钟充满。",
        "华星耳机X1：主动降噪，续航30小时（含充电盒），蓝牙5.3，支持多点连接，单只重量5g。",
        "退换货政策：收货7天内无理由退货，15天内换货，需保持原包装完好。",
        "质保政策：手机整机保修1年，电池保修6个月，人为损坏不在保修范围。",
        "维修收费：屏幕更换299元，电池更换99元，主板维修视情况报价。",
        "发货时间：工作日下午3点前付款当天发货，之后次日发货，节假日顺延。",
        "物流查询：发货后收短信通知，可在「我的订单」查看，或拨打400-888-8888。",
        "配送范围：支持全国配送（港澳台除外），偏远地区额外3-5个工作日。",
    ];

    let embedding_model = ollama.embedding_model_with_ndims("qwen3-embedding:0.6b", 1024);

    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(knowledge_base)?
        .build()
        .await?;

    let mut vector_store = InMemoryVectorStore::default();

    vector_store.add_documents(embeddings);

    let index = vector_store.index(embedding_model);

    let memory = Arc::new(InMemoryConversationMemory::default());

    let agent = openai
        .agent("gpt-5.4-mini")
        .preamble(
            "你是华星科技的智能客服助手小星。\
            职责是解答产品规格、售后政策和物流查询等问题。\
            回答保持友好、简洁、准确。 \
            知识库中找不到答案时，建议用户拨打400-888-8888。 \
            不要编造任何产品参数或政策内容。",
        )
        .dynamic_context(2, index)
        .temperature(0.3)
        .memory(memory)
        .conversation("user_001")
        .build();

    println!("===华星科技智能客服===");
    println!("(输入 quit 退出)的 \n");

    let questions = vec![
        "你好，华星 Pro 的电池容量是多少？",
        "那充满电大概需要多长时间？", // 这个问题依赖上一轮的上下文
        "华星 Lite 和 Pro 相比，主要差在哪里？",
        "如果我买了 Pro，屏幕摔碎了，保修能换吗？",
        "不在保修范围的话，换屏幕要多少钱？",
    ];

    for question in questions {
        println!("用户: {}", question);
        println!("小星");

        let mut stream = agent.stream_prompt(question).await;
        stream_to_stdout(&mut stream).await?;
        println!("\n");
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
