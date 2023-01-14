use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// work address, accept ip, subnet mask, ip segment.
    #[arg(short, long)]
    address: String,

    /// work port, you can enter a single port, port set, and port range.
    #[arg(short, long)]
    port: Option<String>,
}
