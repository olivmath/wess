use colored::Colorize;
use std::error::Error;
use term_size::dimensions;
use textwrap::fill;

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