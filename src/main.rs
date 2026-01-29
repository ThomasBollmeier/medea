use std::io::Read;
use clap::Parser;
use medea;

#[derive(Debug, Parser)]
#[command(name = "medea")]
#[command(author, version)]
#[command(about = "a simple pretty-printer for JSON data", long_about = None)]
#[command(help_template = "{name} - {about} [version: {version}, author: {author}]\n\n{usage-heading} {usage}\n\n{all-args}")]
struct Cli {
    /// Input JSON file (optional, defaults to stdin)
    #[arg(default_value = "")]
    json_file: String,

    /// Indent size for pretty-printing
    #[arg(short, long, default_value_t = 4)]
    indent: usize,

    /// Use colors in output
    #[arg(short='c', long, default_value_t = false)]
    use_colors: bool,
}


fn main() -> anyhow::Result<()> {

    let cli = Cli::parse();

    let input_json = if !cli.json_file.is_empty() {
        std::fs::read_to_string(&cli.json_file)
            .map_err(|e| anyhow::anyhow!("Failed to read input file {}: {}", cli.json_file, e))?
    } else {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {}", e))?;
        buffer
    };

    let pretty_json = medea::pretty_print_json(&input_json, cli.indent, cli.use_colors)?;

    for colored_str in pretty_json {
        print!("{}", colored_str);
    }

    Ok(())
}
