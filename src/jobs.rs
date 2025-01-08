use abscissa_core::{
    tracing::{info, warn},
    Application,
};
use tokio::task::JoinHandle;

use crate::{prelude::APP, utils::get_latest_block_height};

pub(crate) async fn start_jobs_thread() -> JoinHandle<()> {
    tokio::spawn(async {
        let latest_block_height = get_latest_block_height(APP.config().cosmos.grpc.clone()).await;
        if latest_block_height.is_err() {
            warn!("failed to get latest block height. running jobs just in case.");
        }

        // For testing purposes, we can set these values in config. By default the hardcoded values will be used.
        let (target_height, cellars_v2_2, cellars_v2_5) =
            match APP.config().jobs.update_strategist_platform_cut.clone() {
                Some(config) => (config.height, config.cellars_v2_2, config.cellars_v2_5),
                None => (
                    update_strategist_platform_cut::TARGET_HEIGHT,
                    vec![update_strategist_platform_cut::RYETH_CELLAR_ADDRESS.to_string()],
                    vec![update_strategist_platform_cut::TURBO_STETH_CELLAR_ADDRESS.to_string()],
                ),
            };

        if latest_block_height.is_err() || latest_block_height.unwrap() < target_height {
            if let Err(err) =
                update_strategist_platform_cut::run(target_height, cellars_v2_2, cellars_v2_5).await
            {
                warn!("job failed: update strategist platform cut: {:?}", err);
            }
        }

        info!("jobs completed");
    })
}

mod update_strategist_platform_cut {
    use abscissa_core::tracing::info;
    use ethers::abi::AbiEncode;
    use somm_proto::cork::Cork;

    use crate::{
        abi::{
            cellar_v2_2::{
                CellarV2_2Calls, SetStrategistPlatformCutCall as V2_2_SetStrategistPlatformCutCall,
            },
            cellar_v2_5::{
                CellarV2_5Calls, SetStrategistPlatformCutCall as V2_5_SetStrategistPlatformCutCall,
            },
        },
        error::Error,
        somm_send,
    };

    pub(crate) const TARGET_HEIGHT: u64 = 18000000;
    pub(crate) const RYETH_CELLAR_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
    pub(crate) const TURBO_STETH_CELLAR_ADDRESS: &str =
        "0x0000000000000000000000000000000000000000";

    pub(crate) async fn run(
        target_height: u64,
        cellars_v2_2: Vec<String>,
        cellars_v2_5: Vec<String>,
    ) -> Result<(), Error> {
        schedule_2_5_corks(target_height, cellars_v2_5).await?;
        schedule_2_2_corks(target_height, cellars_v2_2).await?;

        Ok(())
    }

    async fn schedule_2_5_corks(
        target_height: u64,
        cellars_v2_5: Vec<String>,
    ) -> Result<(), Error> {
        let call = set_strategist_platform_cut_2_5_call()?;
        for cellar_address in cellars_v2_5 {
            let cork = Cork {
                encoded_contract_call: call.clone(),
                target_contract_address: cellar_address.clone(),
            };

            let response = somm_send::schedule_cork(cork, target_height).await?;
            info!(
                "response from scheduling cork for {}: {:?}",
                cellar_address, response
            );
        }

        Ok(())
    }

    async fn schedule_2_2_corks(
        target_height: u64,
        cellars_v2_2: Vec<String>,
    ) -> Result<(), Error> {
        let call = set_strategist_platform_cut_2_2_call()?;
        for cellar_address in cellars_v2_2 {
            let cork = Cork {
                encoded_contract_call: call.clone(),
                target_contract_address: cellar_address.clone(),
            };

            let response = somm_send::schedule_cork(cork, target_height).await?;
            info!(
                "response from scheduling cork for {}: {:?}",
                cellar_address, response
            );
        }

        Ok(())
    }

    fn set_strategist_platform_cut_2_2_call() -> Result<Vec<u8>, Error> {
        let call = V2_2_SetStrategistPlatformCutCall {
            cut: 500_000_000_000_000_000u64,
        };

        Ok(CellarV2_2Calls::SetStrategistPlatformCut(call).encode())
    }

    fn set_strategist_platform_cut_2_5_call() -> Result<Vec<u8>, Error> {
        let call = V2_5_SetStrategistPlatformCutCall {
            // 50%, max value is 1e18
            cut: 500_000_000_000_000_000u64,
        };

        Ok(CellarV2_5Calls::SetStrategistPlatformCut(call).encode())
    }
}
