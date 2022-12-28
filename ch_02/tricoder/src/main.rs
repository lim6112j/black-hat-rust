use std::{env, time::Duration};
mod error;
pub use error::Error;
mod subdomains;
mod ports;
mod model;
mod common_ports;
fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        return Err(Error::CliUsage.into());
    }
    Ok(())
}
