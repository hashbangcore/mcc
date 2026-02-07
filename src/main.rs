mod core;
mod task;
mod utils;

use clap::Parser;
use core::interfaz;
use std::env;
use task::prompt;

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
        Some(interfaz::Commands::Commit { hint }) => generate_commit(ctx, hint.as_deref()).await?,
        Some(interfaz::Commands::Prompt { input }) => prompt::send_chat(ctx, &input).await?,
        None => {
            if let Some(prompt) = args.prompt {
                prompt::send_chat(ctx, &prompt).await?;
            } else {
                eprintln!("Error: a message is required for chat or commit");
            }
        }
    }

    Ok(())
}

async fn generate_commit(
    ctx: &core::CliContext,
    hint: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let prompt = task::commit::prompt::generate(hint);

    if ctx.verbose {
        println!("{}\n\n", prompt);
    }

    let result = ctx.ai.complete(&prompt).await?;

    println!("{}", result);

    Ok(())
}
