use evo_agent_sdk::{AgentRunner, kernel_handlers::PreLoadHandler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    AgentRunner::run(PreLoadHandler).await
}
