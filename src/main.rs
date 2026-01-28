use clap::Parser;
use medea;

#[derive(Debug, Parser)]
#[command(name = "json_tool")]
#[command(about = "A simple JSON parser and pretty-printer", long_about = None)]
struct Cli {
    /// Input JSON file
    json_file: String,

    /// Indent size for pretty-printing
    #[arg(short, long, default_value_t = 4)]
    indent: usize,
}


fn main() -> anyhow::Result<()> {

    let cli = Cli::parse();

    let input_json = std::fs::read_to_string(&cli.json_file)
        .map_err(|e| anyhow::anyhow!("Failed to read input file {}: {}", cli.json_file, e))?;

    let pretty_json = medea::pretty_print_json(&input_json, cli.indent)?;

    println!("{}", pretty_json);

    Ok(())
}
