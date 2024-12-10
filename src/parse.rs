use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Text to translate
    #[arg(index = 1, value_name = "text")]
    pub text: String,

    /// Your Youdao App ID
    #[arg(long, env = "YD_APP_ID", default_value_t = String::new())]
    pub yd_id: String,

    /// Your Youdao App Key
    #[arg(long, env = "YD_APP_KEY", default_value_t = String::new())]
    pub yd_key: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
