use futures::{stream, StreamExt};
use reqwest::{redirect, Client};
use std::sync::Arc;
use std::time::Instant;
use std::{env, time::Duration};
use tokio::sync::Mutex;
mod error;
pub use error::Error;
mod common_ports;
mod model;
use model::Subdomain;
mod ports;
mod subdomains;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();
    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;
    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let scan_start = Instant::now();
    let subdomains = subdomains::enumerate(&http_client, target).await?;
    let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new()));
    stream::iter(subdomains.into_iter())
        .for_each_concurrent(subdomains_concurrency, |subdomain| {
            let res = res.clone();
            async move {
                let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
                println!("{}", &subdomain.domain);
                for port in &subdomain.open_ports {
                    println!("    {}: open", port.port);
                }
                res.lock().await.push(subdomain)
            }
        })
        .await;
    let scan_duration = scan_start.elapsed();
    println!("Scan completed in {:?}", scan_duration);
    Ok(())
}
