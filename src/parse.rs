use clap::Parser;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Text to translate
    #[arg(index = 1, value_name = "text")]
    pub text: Option<String>,

    /// Your Youdao App ID
    #[arg(long, env = "YD_APP_ID", default_value_t = String::new())]
    pub yd_id: String,

    /// Your Youdao App Key
    #[arg(long, env = "YD_APP_KEY", default_value_t = String::new())]
    pub yd_key: String,
}

pub fn parse_args() -> Args {
    let mut args = Args::parse();

    if args.text.is_none() {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from pipe");
        args.text = Some(buffer);
    }

    args
}
