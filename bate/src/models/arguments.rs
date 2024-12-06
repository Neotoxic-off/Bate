use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments
{
    #[arg(short, long, required = true)]
    pub file: String,

    #[arg(short, long, required = true)]
    pub keys: String,

    #[arg(long, default_value = "1", required = false)]
    pub minimum: u64,

    #[arg(long, default_value = "16", required = false)]
    pub maximum: u64,

    #[arg(short, long, default_value = "0.7", required = false)]
    pub score: f64
}
