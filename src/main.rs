use anyhow::Result;
use clap::Parser;
use rust_xdiff::{
    cli::{Action, Args, RunArgs},
    DiffConfig,
};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    // Args { action: Run(RunArgs { profile: "rust",
    // extra_params: [KeyVal { key_type: Query, key: "a", value: "100" }],
    // config: Some("fixtures/test.yml") }) }
    println!("{:?}\n", args);

    match args.action {
        Action::Run(args) => run(args).await?,
        _ => panic!("Not implemented"),
    }

    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    let config_file = args.config.unwrap_or_else(|| "./xdiff.yml".to_string());
    let config = DiffConfig::load_yaml(&config_file).await?;
    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow::anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;

    let extra_args = args.extra_params.into();
    let output = profile.diff(extra_args).await?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", output)?;

    Ok(())
}
