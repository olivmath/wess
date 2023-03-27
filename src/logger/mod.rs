use colored::Colorize;
use std::error::Error;
use term_size::dimensions;
use textwrap::fill;

/// # Logs the given text to stdout.
/// Wrapped and formatted according to the terminal width.
///
/// The input text is wrapped based on the terminal width,
/// and the wrapped text is printed with a green line above and below.
/// If the terminal width cannot be determined, a default
/// width of 80 characters is used.
///
/// # Arguments
///
/// * `text` - The text to be logged to stdout.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error + Send + Sync>>` - Returns an empty `Result` on successful logging.
///   In case of any error, it returns an `Error` trait object.
///
/// # Errors
///
/// This function might return an error if there's an issue with stdout.
///
/// # Example
///
/// ```
/// stdout_log("This is a sample log message.").await.unwrap();
/// ```
///
/// # Result in Terminal
///
/// ```
/// // ------------------------------------------------------------
/// // This is a sample log message.
/// // ------------------------------------------------------------
/// ```
pub async fn stdout_log(text: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let width = dimensions().unwrap_or((80, 24)).0; // obtém a largura atual do terminal ou usa 80 como padrão
    let wrapped_text = fill(text, width); // quebra a string em várias linhas de acordo com a largura

    let line = "-".repeat(width);
    println!("{}", line.green());
    for line in wrapped_text.lines() {
        println!("{}", line.white());
    }
    println!("{}", line.green());

    Ok(())
}

/// Clear terminal
pub fn clear_terminal_with(string: &str) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", string);
}

/// simple error logs
pub fn log_err(message: String) {
    eprintln!("{message}");
}
