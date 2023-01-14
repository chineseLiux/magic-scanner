#[macro_use]
extern crate log;
extern crate colorful;

use crate::args::Args;
use crate::logger::Logger;
use clap::Parser;

mod args;
mod logger;

#[tokio::main]
async fn main() {
    Logger::default().init().print_banner();
    let args = Args::parse();
    info!("args: {:?}", &args);
}
