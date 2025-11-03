use clap::{Parser, Subcommand};
use tracing_subscriber::fmt::format::FmtSpan;
mod commands {
    pub mod agents;
    pub mod follow;
    pub mod profit;
    pub mod status;
    pub mod telegram;
}
#[derive(Parser)]
#[command(name = "nof1-trade")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Agents,
    Follow {
        agent: String,
        #[arg(long, default_value_t = 30)]
        interval: u64,
        #[arg(long, default_value_t = 0.5)]
        price_tolerance: f64,
        #[arg(long, default_value_t = 10.0)]
        total_margin: f64,
        #[arg(long)]
        risk_only: bool,
        #[arg(long)]
        margin_type: Option<String>,
    },
    Profit,
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Agents => {
            commands::agents::run_agents_command();
        }
        Commands::Follow {
            agent,
            interval,
            price_tolerance,
            total_margin,
            risk_only,
            margin_type,
        } => {
            commands::follow::run_follow_command();
        }
        Commands::Profit => {
            commands::profit::run_profit_command();
        }
        Commands::Status => {
            commands::status::run_status_command();
        }
    }

    Ok(())
}
