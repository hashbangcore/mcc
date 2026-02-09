mod core;
mod tasks;
mod utils;

use clap::CommandFactory;
use clap::Parser;
use clap_complete::generate;
use core::interfaz;
use tasks::chat;
use tasks::commit;
use tasks::message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = utils::get_stdin();
    let args = interfaz::Cli::parse();
    let ai = core::Service::new(Some(&args.provider));

    let ctx = core::CliContext {
        ai,
        stdin,
        verbose: args.verbose,
        provider: args.provider.to_string(),
    };

    execute(&ctx, args).await?;

    Ok(())
}

async fn execute(
    ctx: &core::CliContext,
    args: interfaz::Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Some(interfaz::Commands::Commit { hint }) => commit::generate(ctx, hint.as_deref()).await?,
        Some(interfaz::Commands::Prompt { input }) => message::generate(ctx, &input).await?,
        Some(interfaz::Commands::Chat) => chat::generate(ctx).await,
        Some(interfaz::Commands::Completion { shell }) => {
            let mut cmd = interfaz::Cli::command();
            generate(shell, &mut cmd, "netero", &mut std::io::stdout());
        }
        None => {
            if let Some(prompt) = args.prompt {
                message::generate(ctx, &prompt).await?;
            } else {
                chat::generate(ctx).await;
            }
        }
    }

    Ok(())
}
