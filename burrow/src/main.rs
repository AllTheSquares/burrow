use clap::{Args, Parser, Subcommand};
use tokio::io::Result;
use tun::TunInterface;

mod daemon;

use daemon::{DaemonClient, DaemonCommand, DaemonStartOptions};

#[derive(Parser)]
#[command(name = "Burrow")]
#[command(author = "Hack Club <team@hackclub.com>")]
#[command(version = "0.1")]
#[command(
    about = "Burrow is a tool for burrowing through firewalls, built by teenagers at Hack Club.",
    long_about = "Burrow is a 🚀 blazingly fast 🚀 tool designed to penetrate unnecessarily restrictive firewalls, providing teenagers worldwide with secure, less-filtered, and safe access to the internet!
It's being built by teenagers from Hack Club, in public! Check it out: https://github.com/hackclub/burrow
Spotted a bug? Please open an issue! https://github.com/hackclub/burrow/issues/new"
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start Burrow
    Start(StartArgs),
    /// Stop Burrow daemon
    Stop,
    /// Start Burrow daemon
    Daemon(DaemonArgs),
}

#[derive(Args)]
struct StartArgs {}

#[derive(Args)]
struct DaemonArgs {}

async fn try_main() -> Result<()> {
    burrow::ensureroot::ensure_root();

    let iface = TunInterface::new()?;
    println!("{:?}", iface.name());

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    println!("Platform: {}", std::env::consts::OS);

    let cli = Cli::parse();
    match &cli.command {
        Commands::Start(..) => {
            if cfg!(target_family = "unix") {
                let mut client = DaemonClient::new().await?;
                client
                    .send_command(DaemonCommand::Start(DaemonStartOptions::default()))
                    .await?;
            } else {
                try_main().await?;
            }
        }
        Commands::Stop => {
            if cfg!(target_family = "unix") {
                let mut client = DaemonClient::new().await?;
                client.send_command(DaemonCommand::Stop).await?;
            } else {
                try_main().await?;
            }
        }
        Commands::Daemon(_) => daemon::daemon_main().await?,
    }

    Ok(())
}
