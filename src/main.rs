// main.rs
mod parse;
mod yd;
mod output;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse::parse_args();

    let translation = yd::translate(&args.yd_id, &args.yd_key, &args.text).await;
    let exit_code = output::handle_error(translation);
    std::process::exit(exit_code);
}
