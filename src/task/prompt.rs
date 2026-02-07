use crate::core;
use crate::utils;
use std::env;

/// Envía una solicitud al LLM usando el contexto de la CLI.
///
/// Esta función combina el prompt del usuario, el contenido de stdin
/// (si existe) y un preámbulo con información de usuario y fecha,
/// luego llama al servicio AI para obtener la respuesta.
///
/// # Argumentos
/// - `ctx`: Referencia al contexto de la CLI, que incluye AI, stdin, verbose y provider.
/// - `request`: La solicitud de usuario a enviar al modelo.
///
/// # Errores
/// Devuelve `Box<dyn std::error::Error>` si ocurre un error al completar la solicitud.
pub async fn send_chat(
    ctx: &core::CliContext,
    request: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let preamble = format!(
        "LLM name: Netero\nUser name: {}\nDate and hour: {}\n",
        utils::capitalize(&user),
        utils::current_datetime()
    );

    let prompt = if ctx.stdin.trim().is_empty() {
        format!("User request:\n {}\n", request.trim())
    } else {
        format!(
            "== USER REQUEST ==\n{}\n== END USER REQUEST ==\n\n== STDIN FILE ==\n{}\n== END STDIN FILE ==\n",
            request.trim(),
            ctx.stdin.trim()
        )
    };

    let wrapper = format!("{}\n{}", preamble, prompt);

    let response = ctx.ai.complete(&wrapper).await?;

    if ctx.verbose {
        println!("\x1b[1m{}:\x1b[0m\n\n{}\n", user.to_uppercase(), wrapper);
        println!(
            "\x1b[1m{}:\x1b[0m\n\n{}",
            ctx.provider.to_uppercase(),
            response.trim()
        );
    } else {
        println!("{}", response.trim());
    }

    Ok(())
}
