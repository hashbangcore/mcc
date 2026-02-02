mod core;
mod task;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rave")]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    texto: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Commit { hint: Option<String> },
    Chat { texto: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ai = core::Model::new();
    let args = Cli::parse();

    execute(&ai, args).await?;

    Ok(())
}

async fn execute(ai: &core::Model, args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Some(Commands::Commit { hint }) => generate_commit(ai, hint.as_deref()).await?,

        Some(Commands::Chat { texto }) => send_chat(ai, &texto).await?,

        None => {
            if let Some(texto) = args.texto {
                send_chat(ai, &texto).await?;
            } else {
                eprintln!("Error: a message is required for chat or commit");
            }
        }
    }

    Ok(())
}

async fn generate_commit(
    ai: &core::Model,
    hint: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let prompt = task::commit::prompt::generate(hint);

    let result = ai.complete(&prompt).await?;
    println!("# PROMPT");
    println!("{}", prompt);
    println!("# RESPONSE");
    println!("{}", result);

    Ok(())
}

async fn send_chat(ai: &core::Model, mensaje: &str) -> Result<(), Box<dyn std::error::Error>> {
    let respuesta = ai.complete(mensaje).await?;
    println!("{}", respuesta);
    Ok(())
}
