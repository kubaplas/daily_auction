//! Deploys the daily auction and lets you call it from the command line.

use daily_auction::auction::{DailyAuction, DailyAuctionInitArgs};
use odra::host::{HostEnv, InstallConfig};
use odra_cli::{
    cspr, deploy::DeployScript, DeployedContractsContainer, DeployerExt, OdraCli,
};

/// Round length used by the deploy script, in milliseconds. Override with `ROUND_DURATION_MS`.
const DAY_MS: u64 = 24 * 60 * 60 * 1000;

pub struct DailyAuctionDeployScript;

impl DeployScript for DailyAuctionDeployScript {
    fn deploy(
        &self,
        env: &HostEnv,
        container: &mut DeployedContractsContainer,
    ) -> Result<(), odra_cli::deploy::Error> {
        let round_duration = std::env::var("ROUND_DURATION_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DAY_MS);
        let _auction = DailyAuction::load_or_deploy_with_cfg(
            env,
            None,
            DailyAuctionInitArgs {
                name: "Daily Auction".to_string(),
                symbol: "DAILY".to_string(),
                round_duration,
                min_bid: cspr!(1).into(),
            },
            InstallConfig::upgradable::<DailyAuction>(),
            container,
            cspr!(500),
        )?;
        Ok(())
    }
}

pub fn main() {
    OdraCli::new()
        .about("CLI tool for the daily_auction smart contract")
        .deploy(DailyAuctionDeployScript)
        .contract::<DailyAuction>()
        .build()
        .run();
}
