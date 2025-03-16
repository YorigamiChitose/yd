mod output;
mod parse;
mod yd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse::parse_args();

    let text = match args.text {
        Some(t) => t,
        None => {
            eprintln!("No text provided. Use piped input or provide text as an argument.");
            std::process::exit(1);
        }
    };

    let translation = yd::translate(&args.yd_id, &args.yd_key, &text).await;
    let exit_code = output::handle_error(translation);
    std::process::exit(exit_code);
}
