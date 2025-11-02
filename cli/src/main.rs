use clap::{Parser, Subcommand};
use tracing_subscriber::fmt::format::FmtSpan;

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
            println!("Agents command (stub)");
        }
        Commands::Follow {
            agent,
            interval,
            price_tolerance,
            total_margin,
            risk_only,
            margin_type,
        } => {
            println!("Following agent: {}", agent);
            println!(
                "interval: {}s, price_tolerance: {}%, total_margin: {}, risk_only: {}, margin_type: {:?}",
                interval, price_tolerance, total_margin, risk_only, margin_type
            );
            // TODO: wire to FollowService in services crate
        }
        Commands::Profit => {
            println!("Profit command (stub)");
        }
        Commands::Status => {
            println!("Status command (stub)");
        }
    }

    Ok(())
}
