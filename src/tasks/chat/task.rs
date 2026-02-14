use crate::core;
use crate::tasks::render;
use crate::utils;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use super::commands::{
    CommandCompleter, handle_add, handle_clean, handle_eval, handle_help, handle_stream,
    handle_trans,
};
use super::inline_exec::run_inline_commands;
use super::parse::strip_inline_commands;
use super::prompt::create_prompt;
use super::stream::stream_completion;

/// Starts the interactive chat session and handles all supported commands.
pub async fn generate_chat(
    service: &core::Service,
    args: &core::Cli,
    stdin: String,
    stdin_is_piped: bool,
) {
    let mut history: Vec<String> = Vec::new();
    let mut pending_stdin = if stdin.trim().is_empty() {
        None
    } else {
        Some(stdin)
    };
    let mut stream_enabled = false;
    let mut rl = Editor::<CommandCompleter, DefaultHistory>::new()
        .expect("failed to initialize rustyline editor");
    rl.set_helper(Some(CommandCompleter::new(vec![
        "/clean", "/trans", "/eval", "/help", "/stream", "/add",
    ])));
    let mut tty_reader = if stdin_is_piped {
        match File::open("/dev/tty") {
            Ok(file) => Some(BufReader::new(file)),
            Err(err) => {
                eprintln!("Error: {}", err);
                return;
            }
        }
    } else {
        None
    };

    loop {
        let user_input = if let Some(reader) = tty_reader.as_mut() {
            let mut stdout = std::io::stdout();
            if stdout
                .write_all(b"\x1b[36m\xE2\x9E\x9C ")
                .is_err()
            {
                break;
            }
            if stdout.flush().is_err() {
                break;
            }
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if stdout.write_all(b"\x1b[0m").is_err() {
                        break;
                    }
                    line.trim().to_string()
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            }
        } else {
            println!("\x1b[36m");
            let readline = rl.readline("➜ ");
            let user_input = match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str()).unwrap();
                    line.trim().to_string()
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            };
            println!("\x1b[0m");
            user_input
        };

        if user_input.is_empty() {
            continue;
        }

        if handle_clean(&user_input, &mut history) {
            continue;
        }

        if handle_help(&user_input) {
            continue;
        }

        if handle_add(&user_input, &mut history, &mut pending_stdin) {
            continue;
        }

        if handle_stream(&user_input, &mut stream_enabled) {
            continue;
        }

        match handle_trans(&user_input, service, args).await {
            Ok(true) => continue,
            Ok(false) => {}
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        }

        if handle_eval(&user_input) {
            continue;
        }

        let dialog = history.join("\n");
        let command_output = run_inline_commands(&user_input);
        let cleaned_input = strip_inline_commands(&user_input);
        let prompt = create_prompt(
            &utils::get_user(),
            &utils::current_datetime(),
            &utils::get_user_lang(),
            &dialog,
            &cleaned_input,
            command_output.as_deref(),
            pending_stdin.as_deref(),
        );
        if pending_stdin.is_some() {
            pending_stdin = None;
        }

        if args.verbose {
            println!("\x1b[32m{}\x1b[0m", prompt);
        }

        let response = if stream_enabled {
            match stream_completion(service, &prompt).await {
                Ok(text) => text,
                Err(err) => {
                    eprintln!("AI error: {}", err);
                    break;
                }
            }
        } else {
            match service.complete(&prompt).await {
                Ok(text) => {
                    let output = render::render_markdown(&text);
                    println!("\n{}", output);
                    text
                }
                Err(err) => {
                    eprintln!("AI error: {}", err);
                    break;
                }
            }
        };

        history.push(format!("{}: {}", utils::get_user(), cleaned_input));
        history.push(format!("Assistant: {}\n", response));
    }
}
