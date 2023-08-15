use anyhow::Result;
use rust_xdiff::{LoadConfig, RequestConfig};
// use rust_xdiff::DiffConfig;

fn main() -> Result<()> {
    let content = include_str!("../fixtures/xreq_test.yml");
    // let config = DiffConfig::from_yaml(content)?;
    let config = RequestConfig::from_yaml(content)?;

    println!("config: {:#?}", config);
    Ok(())
}
