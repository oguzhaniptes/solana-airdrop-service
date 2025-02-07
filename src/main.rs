use daemonize::Daemonize;
use log::{error, info};
use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::{metadata, File, OpenOptions};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use which::which;

const AIRDROP_AMOUNT: &str = "1";
const TARGET_ADDRESS: &str = "your-wallet-address";
const LOG_FILE: &str = "/tmp/solana_airdrop.log";
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10 MB
const TEST_MODE: bool = false;

/// Check log file size and reset if it's too big
fn check_and_reset_log() {
    if let Ok(metadata) = metadata(LOG_FILE) {
        if metadata.len() > MAX_LOG_SIZE {
            info!("🔄💾 Log file is too big! Resetting...");
            let _ = File::create(LOG_FILE);
        }
    }
}

/// Run Solana airdrop command
async fn request_airdrop() -> Result<(), Box<dyn std::error::Error>> {
    check_and_reset_log();

    if let Ok(solana_path) = which("solana") {
        info!("📂 Solana CLI found: {:?}", solana_path);

        let output = Command::new(solana_path)
            .arg("airdrop")
            .arg(AIRDROP_AMOUNT)
            .arg(TARGET_ADDRESS)
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    info!("✅🎉🥳 Airdrop sent successfully!");
                    Ok(())
                } else {
                    error!("❌ Airdrop failed: {:?}", result);
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "😔 Airdrop failed",
                    )))
                }
            }
            Err(e) => {
                error!("❌ Command execution error: {}", e);
                Ok(())
            }
        }
    } else {
        error!("📁🥺 Solana CLI not found!");
        Ok(())
    }
}

async fn wait_until_midnight() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("🕒 Time information not available");

    let seconds_since_midnight = (now.as_secs() % 86400) as u64;
    let seconds_until_midnight = 86400 - seconds_since_midnight;

    info!(
        "🕛 Waiting for 00:00... {} seconds...",
        seconds_until_midnight
    );
    sleep(Duration::from_secs(seconds_until_midnight)).await;
}

#[tokio::main]
async fn main() {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("💾 Log file not found!");
    WriteLogger::init(LevelFilter::Info, Config::default(), log_file)
        .expect("💾 Log file not initialized!");

    let daemonize = Daemonize::new()
        .pid_file("/tmp/solana_airdrop.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .umask(0o027);

    match daemonize.start() {
        Ok(_) => info!("✅📦 Service started!"),
        Err(e) => {
            error!("❌📦 Service not started: {}", e);
            return;
        }
    }

    // ⏳ First run wait until 00:00
    wait_until_midnight().await;

    loop {
        info!("⏳ Airdrop process starting...");

        for i in 0..2 {
            info!("🚀👀 Sending {}. airdrop request...", i + 1);

            match request_airdrop().await {
                Ok(_) => info!("✅⛏️ {}. airdrop request sent successfully!", i + 1),
                Err(e) => error!("❌😢 {} Airdrop request failed: {}", i + 1, e),
            }

            sleep(Duration::from_secs(10)).await;
        }

        if TEST_MODE {
            info!("⏸️⏳ Waiting for 1 minute (Test Mode)...");
            sleep(Duration::from_secs(60)).await;
        } else {
            info!("⏸️⏳ Waiting for 8 hours...");
            sleep(Duration::from_secs(8 * 3600)).await;
        }
    }
}
