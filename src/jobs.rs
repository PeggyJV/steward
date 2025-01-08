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

        if latest_block_height.is_err()
            || latest_block_height.unwrap() < update_strategist_platform_cut::TARGET_HEIGHT
        {
            if let Err(err) = update_strategist_platform_cut::run().await {
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
    const RYETH_CELLAR_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
    const TURBO_STETH_CELLAR_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

    pub(crate) async fn run() -> Result<(), Error> {
        schedule_ryeth_cork().await?;
        schedule_turbo_steth_cork().await?;

        Ok(())
    }

    async fn schedule_ryeth_cork() -> Result<(), Error> {
        let call = set_platform_fee_split_2_2_call()?;
        let cork = Cork {
            encoded_contract_call: call,
            target_contract_address: RYETH_CELLAR_ADDRESS.to_string(),
        };

        let response = somm_send::schedule_cork(cork, TARGET_HEIGHT).await?;

        info!("response from scheduling ryeth cork: {:?}", response);

        Ok(())
    }

    async fn schedule_turbo_steth_cork() -> Result<(), Error> {
        let call = set_strategist_platform_cut_2_5_call()?;
        let cork = Cork {
            encoded_contract_call: call,
            target_contract_address: TURBO_STETH_CELLAR_ADDRESS.to_string(),
        };

        let response = somm_send::schedule_cork(cork, TARGET_HEIGHT).await?;

        info!("response from scheduling turbo steth cork: {:?}", response);

        Ok(())
    }

    fn set_platform_fee_split_2_2_call() -> Result<Vec<u8>, Error> {
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
