pub use uniswapv3_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswapv3_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "UniswapV3 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPV3_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  { \"inputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"constructor\" },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Burn\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"Collect\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"CollectProtocol\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"paid0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"paid1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Flash\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNextOld\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNextNew\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"IncreaseObservationCardinalityNext\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceX96\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int24\",\n        \"name\": \"tick\",\n        \"type\": \"int24\"\n      }\n    ],\n    \"name\": \"Initialize\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Mint\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol0Old\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol1Old\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol0New\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol1New\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"SetFeeProtocol\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int256\",\n        \"name\": \"amount0\",\n        \"type\": \"int256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int256\",\n        \"name\": \"amount1\",\n        \"type\": \"int256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceX96\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int24\",\n        \"name\": \"tick\",\n        \"type\": \"int24\"\n      }\n    ],\n    \"name\": \"Swap\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"int24\", \"name\": \"tickLower\", \"type\": \"int24\" },\n      { \"internalType\": \"int24\", \"name\": \"tickUpper\", \"type\": \"int24\" },\n      { \"internalType\": \"uint128\", \"name\": \"amount\", \"type\": \"uint128\" }\n    ],\n    \"name\": \"burn\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amount0\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount1\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"int24\", \"name\": \"tickLower\", \"type\": \"int24\" },\n      { \"internalType\": \"int24\", \"name\": \"tickUpper\", \"type\": \"int24\" },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0Requested\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1Requested\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"collect\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"amount0\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"amount1\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0Requested\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1Requested\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"collectProtocol\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"amount0\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"amount1\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"factory\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"fee\",\n    \"outputs\": [{ \"internalType\": \"uint24\", \"name\": \"\", \"type\": \"uint24\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeGrowthGlobal0X128\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeGrowthGlobal1X128\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount0\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount1\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"flash\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNext\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"increaseObservationCardinalityNext\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint160\", \"name\": \"sqrtPriceX96\", \"type\": \"uint160\" }\n    ],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"liquidity\",\n    \"outputs\": [{ \"internalType\": \"uint128\", \"name\": \"\", \"type\": \"uint128\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"maxLiquidityPerTick\",\n    \"outputs\": [{ \"internalType\": \"uint128\", \"name\": \"\", \"type\": \"uint128\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"int24\", \"name\": \"tickLower\", \"type\": \"int24\" },\n      { \"internalType\": \"int24\", \"name\": \"tickUpper\", \"type\": \"int24\" },\n      { \"internalType\": \"uint128\", \"name\": \"amount\", \"type\": \"uint128\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"mint\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amount0\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount1\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"name\": \"observations\",\n    \"outputs\": [\n      { \"internalType\": \"uint32\", \"name\": \"blockTimestamp\", \"type\": \"uint32\" },\n      { \"internalType\": \"int56\", \"name\": \"tickCumulative\", \"type\": \"int56\" },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"secondsPerLiquidityCumulativeX128\",\n        \"type\": \"uint160\"\n      },\n      { \"internalType\": \"bool\", \"name\": \"initialized\", \"type\": \"bool\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint32[]\", \"name\": \"secondsAgos\", \"type\": \"uint32[]\" }\n    ],\n    \"name\": \"observe\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int56[]\",\n        \"name\": \"tickCumulatives\",\n        \"type\": \"int56[]\"\n      },\n      {\n        \"internalType\": \"uint160[]\",\n        \"name\": \"secondsPerLiquidityCumulativeX128s\",\n        \"type\": \"uint160[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"bytes32\", \"name\": \"\", \"type\": \"bytes32\" }],\n    \"name\": \"positions\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"liquidity\", \"type\": \"uint128\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthInside0LastX128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthInside1LastX128\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"uint128\", \"name\": \"tokensOwed0\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"tokensOwed1\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"protocolFees\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"token0\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"token1\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint8\", \"name\": \"feeProtocol0\", \"type\": \"uint8\" },\n      { \"internalType\": \"uint8\", \"name\": \"feeProtocol1\", \"type\": \"uint8\" }\n    ],\n    \"name\": \"setFeeProtocol\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"slot0\",\n    \"outputs\": [\n      { \"internalType\": \"uint160\", \"name\": \"sqrtPriceX96\", \"type\": \"uint160\" },\n      { \"internalType\": \"int24\", \"name\": \"tick\", \"type\": \"int24\" },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationIndex\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinality\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNext\",\n        \"type\": \"uint16\"\n      },\n      { \"internalType\": \"uint8\", \"name\": \"feeProtocol\", \"type\": \"uint8\" },\n      { \"internalType\": \"bool\", \"name\": \"unlocked\", \"type\": \"bool\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"int24\", \"name\": \"tickLower\", \"type\": \"int24\" },\n      { \"internalType\": \"int24\", \"name\": \"tickUpper\", \"type\": \"int24\" }\n    ],\n    \"name\": \"snapshotCumulativesInside\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int56\",\n        \"name\": \"tickCumulativeInside\",\n        \"type\": \"int56\"\n      },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"secondsPerLiquidityInsideX128\",\n        \"type\": \"uint160\"\n      },\n      { \"internalType\": \"uint32\", \"name\": \"secondsInside\", \"type\": \"uint32\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"bool\", \"name\": \"zeroForOne\", \"type\": \"bool\" },\n      { \"internalType\": \"int256\", \"name\": \"amountSpecified\", \"type\": \"int256\" },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceLimitX96\",\n        \"type\": \"uint160\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"swap\",\n    \"outputs\": [\n      { \"internalType\": \"int256\", \"name\": \"amount0\", \"type\": \"int256\" },\n      { \"internalType\": \"int256\", \"name\": \"amount1\", \"type\": \"int256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"int16\", \"name\": \"\", \"type\": \"int16\" }],\n    \"name\": \"tickBitmap\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"tickSpacing\",\n    \"outputs\": [{ \"internalType\": \"int24\", \"name\": \"\", \"type\": \"int24\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"int24\", \"name\": \"\", \"type\": \"int24\" }],\n    \"name\": \"ticks\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"liquidityGross\",\n        \"type\": \"uint128\"\n      },\n      { \"internalType\": \"int128\", \"name\": \"liquidityNet\", \"type\": \"int128\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthOutside0X128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthOutside1X128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"int56\",\n        \"name\": \"tickCumulativeOutside\",\n        \"type\": \"int56\"\n      },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"secondsPerLiquidityOutsideX128\",\n        \"type\": \"uint160\"\n      },\n      { \"internalType\": \"uint32\", \"name\": \"secondsOutside\", \"type\": \"uint32\" },\n      { \"internalType\": \"bool\", \"name\": \"initialized\", \"type\": \"bool\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"token0\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"token1\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct UniswapV3<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapV3<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapV3<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV3))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapV3<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), UNISWAPV3_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `burn` (0xa34123a7) function"]
        pub fn burn(
            &self,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([163, 65, 35, 167], (tick_lower, tick_upper, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collect` (0x4f1eb3d8) function"]
        pub fn collect(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [79, 30, 179, 216],
                    (
                        recipient,
                        tick_lower,
                        tick_upper,
                        amount_0_requested,
                        amount_1_requested,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collectProtocol` (0x85b66729) function"]
        pub fn collect_protocol(
            &self,
            recipient: ethers::core::types::Address,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [133, 182, 103, 41],
                    (recipient, amount_0_requested, amount_1_requested),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fee` (0xddca3f43) function"]
        pub fn fee(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([221, 202, 63, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeGrowthGlobal0X128` (0xf3058399) function"]
        pub fn fee_growth_global_0x128(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([243, 5, 131, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeGrowthGlobal1X128` (0x46141319) function"]
        pub fn fee_growth_global_1x128(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([70, 20, 19, 25], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flash` (0x490e6cbc) function"]
        pub fn flash(
            &self,
            recipient: ethers::core::types::Address,
            amount_0: ethers::core::types::U256,
            amount_1: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 14, 108, 188], (recipient, amount_0, amount_1, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseObservationCardinalityNext` (0x32148f67) function"]
        pub fn increase_observation_cardinality_next(
            &self,
            observation_cardinality_next: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 20, 143, 103], observation_cardinality_next)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xf637731d) function"]
        pub fn initialize(
            &self,
            sqrt_price_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 55, 115, 29], sqrt_price_x96)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidity` (0x1a686502) function"]
        pub fn liquidity(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([26, 104, 101, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxLiquidityPerTick` (0x70cf754a) function"]
        pub fn max_liquidity_per_tick(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([112, 207, 117, 74], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x3c8a7d8d) function"]
        pub fn mint(
            &self,
            recipient: ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [60, 138, 125, 141],
                    (recipient, tick_lower, tick_upper, amount, data),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `observations` (0x252c09d7) function"]
        pub fn observations(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (u32, i64, ethers::core::types::U256, bool)>
        {
            self.0
                .method_hash([37, 44, 9, 215], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `observe` (0x883bdbfd) function"]
        pub fn observe(
            &self,
            seconds_agos: ::std::vec::Vec<u32>,
        ) -> ethers::contract::builders::ContractCall<M, (Vec<i64>, Vec<ethers::core::types::U256>)>
        {
            self.0
                .method_hash([136, 59, 219, 253], seconds_agos)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `positions` (0x514ea4bf) function"]
        pub fn positions(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
                u128,
            ),
        > {
            self.0
                .method_hash([81, 78, 164, 191], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `protocolFees` (0x1ad8b03b) function"]
        pub fn protocol_fees(&self) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([26, 216, 176, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeProtocol` (0x8206a4d1) function"]
        pub fn set_fee_protocol(
            &self,
            fee_protocol_0: u8,
            fee_protocol_1: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 6, 164, 209], (fee_protocol_0, fee_protocol_1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `slot0` (0x3850c7bd) function"]
        pub fn slot_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, i32, u16, u16, u16, u8, bool),
        > {
            self.0
                .method_hash([56, 80, 199, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `snapshotCumulativesInside` (0xa38807f2) function"]
        pub fn snapshot_cumulatives_inside(
            &self,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, (i64, ethers::core::types::U256, u32)>
        {
            self.0
                .method_hash([163, 136, 7, 242], (tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0x128acb08) function"]
        pub fn swap(
            &self,
            recipient: ethers::core::types::Address,
            zero_for_one: bool,
            amount_specified: I256,
            sqrt_price_limit_x96: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, (I256, I256)> {
            self.0
                .method_hash(
                    [18, 138, 203, 8],
                    (
                        recipient,
                        zero_for_one,
                        amount_specified,
                        sqrt_price_limit_x96,
                        data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tickBitmap` (0x5339c296) function"]
        pub fn tick_bitmap(
            &self,
            p0: i16,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([83, 57, 194, 150], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tickSpacing` (0xd0c93a7c) function"]
        pub fn tick_spacing(&self) -> ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([208, 201, 58, 124], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ticks` (0xf30dba93) function"]
        pub fn ticks(
            &self,
            p0: i32,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                i128,
                ethers::core::types::U256,
                ethers::core::types::U256,
                i64,
                ethers::core::types::U256,
                u32,
                bool,
            ),
        > {
            self.0
                .method_hash([243, 13, 186, 147], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token0` (0x0dfe1681) function"]
        pub fn token_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token1` (0xd21220a7) function"]
        pub fn token_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Collect` event"]
        pub fn collect_filter(&self) -> ethers::contract::builders::Event<M, CollectFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CollectProtocol` event"]
        pub fn collect_protocol_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollectProtocolFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Flash` event"]
        pub fn flash_filter(&self) -> ethers::contract::builders::Event<M, FlashFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IncreaseObservationCardinalityNext` event"]
        pub fn increase_observation_cardinality_next_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, IncreaseObservationCardinalityNextFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialize` event"]
        pub fn initialize_filter(&self) -> ethers::contract::builders::Event<M, InitializeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetFeeProtocol` event"]
        pub fn set_fee_protocol_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetFeeProtocolFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UniswapV3Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "Burn",
        abi = "Burn(address,int24,int24,uint128,uint256,uint256)"
    )]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "Collect",
        abi = "Collect(address,address,int24,int24,uint128,uint128)"
    )]
    pub struct CollectFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "CollectProtocol",
        abi = "CollectProtocol(address,address,uint128,uint128)"
    )]
    pub struct CollectProtocolFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "Flash",
        abi = "Flash(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct FlashFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        pub paid_0: ethers::core::types::U256,
        pub paid_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "IncreaseObservationCardinalityNext",
        abi = "IncreaseObservationCardinalityNext(uint16,uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextFilter {
        pub observation_cardinality_next_old: u16,
        pub observation_cardinality_next_new: u16,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "Initialize", abi = "Initialize(uint160,int24)")]
    pub struct InitializeFilter {
        pub sqrt_price_x96: ethers::core::types::U256,
        pub tick: i32,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "Mint",
        abi = "Mint(address,address,int24,int24,uint128,uint256,uint256)"
    )]
    pub struct MintFilter {
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "SetFeeProtocol",
        abi = "SetFeeProtocol(uint8,uint8,uint8,uint8)"
    )]
    pub struct SetFeeProtocolFilter {
        pub fee_protocol_0_old: u8,
        pub fee_protocol_1_old: u8,
        pub fee_protocol_0_new: u8,
        pub fee_protocol_1_new: u8,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,address,int256,int256,uint160,uint128,int24)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
        pub amount_0: I256,
        pub amount_1: I256,
        pub sqrt_price_x96: ethers::core::types::U256,
        pub liquidity: u128,
        pub tick: i32,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV3Events {
        BurnFilter(BurnFilter),
        CollectFilter(CollectFilter),
        CollectProtocolFilter(CollectProtocolFilter),
        FlashFilter(FlashFilter),
        IncreaseObservationCardinalityNextFilter(IncreaseObservationCardinalityNextFilter),
        InitializeFilter(InitializeFilter),
        MintFilter(MintFilter),
        SetFeeProtocolFilter(SetFeeProtocolFilter),
        SwapFilter(SwapFilter),
    }
    impl ethers::contract::EthLogDecode for UniswapV3Events {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(UniswapV3Events::BurnFilter(decoded));
            }
            if let Ok(decoded) = CollectFilter::decode_log(log) {
                return Ok(UniswapV3Events::CollectFilter(decoded));
            }
            if let Ok(decoded) = CollectProtocolFilter::decode_log(log) {
                return Ok(UniswapV3Events::CollectProtocolFilter(decoded));
            }
            if let Ok(decoded) = FlashFilter::decode_log(log) {
                return Ok(UniswapV3Events::FlashFilter(decoded));
            }
            if let Ok(decoded) = IncreaseObservationCardinalityNextFilter::decode_log(log) {
                return Ok(UniswapV3Events::IncreaseObservationCardinalityNextFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(UniswapV3Events::InitializeFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(UniswapV3Events::MintFilter(decoded));
            }
            if let Ok(decoded) = SetFeeProtocolFilter::decode_log(log) {
                return Ok(UniswapV3Events::SetFeeProtocolFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(UniswapV3Events::SwapFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UniswapV3Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3Events::BurnFilter(element) => element.fmt(f),
                UniswapV3Events::CollectFilter(element) => element.fmt(f),
                UniswapV3Events::CollectProtocolFilter(element) => element.fmt(f),
                UniswapV3Events::FlashFilter(element) => element.fmt(f),
                UniswapV3Events::IncreaseObservationCardinalityNextFilter(element) => {
                    element.fmt(f)
                }
                UniswapV3Events::InitializeFilter(element) => element.fmt(f),
                UniswapV3Events::MintFilter(element) => element.fmt(f),
                UniswapV3Events::SetFeeProtocolFilter(element) => element.fmt(f),
                UniswapV3Events::SwapFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(int24,int24,uint128)` and selector `[163, 65, 35, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "burn", abi = "burn(int24,int24,uint128)")]
    pub struct BurnCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    #[doc = "Container type for all input parameters for the `collect`function with signature `collect(address,int24,int24,uint128,uint128)` and selector `[79, 30, 179, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "collect", abi = "collect(address,int24,int24,uint128,uint128)")]
    pub struct CollectCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    #[doc = "Container type for all input parameters for the `collectProtocol`function with signature `collectProtocol(address,uint128,uint128)` and selector `[133, 182, 103, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "collectProtocol",
        abi = "collectProtocol(address,uint128,uint128)"
    )]
    pub struct CollectProtocolCall {
        pub recipient: ethers::core::types::Address,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    #[doc = "Container type for all input parameters for the `factory`function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `fee`function with signature `fee()` and selector `[221, 202, 63, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "fee", abi = "fee()")]
    pub struct FeeCall;
    #[doc = "Container type for all input parameters for the `feeGrowthGlobal0X128`function with signature `feeGrowthGlobal0X128()` and selector `[243, 5, 131, 153]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "feeGrowthGlobal0X128", abi = "feeGrowthGlobal0X128()")]
    pub struct FeeGrowthGlobal0X128Call;
    #[doc = "Container type for all input parameters for the `feeGrowthGlobal1X128`function with signature `feeGrowthGlobal1X128()` and selector `[70, 20, 19, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "feeGrowthGlobal1X128", abi = "feeGrowthGlobal1X128()")]
    pub struct FeeGrowthGlobal1X128Call;
    #[doc = "Container type for all input parameters for the `flash`function with signature `flash(address,uint256,uint256,bytes)` and selector `[73, 14, 108, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "flash", abi = "flash(address,uint256,uint256,bytes)")]
    pub struct FlashCall {
        pub recipient: ethers::core::types::Address,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `increaseObservationCardinalityNext`function with signature `increaseObservationCardinalityNext(uint16)` and selector `[50, 20, 143, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "increaseObservationCardinalityNext",
        abi = "increaseObservationCardinalityNext(uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextCall {
        pub observation_cardinality_next: u16,
    }
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(uint160)` and selector `[246, 55, 115, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint160)")]
    pub struct InitializeCall {
        pub sqrt_price_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `liquidity`function with signature `liquidity()` and selector `[26, 104, 101, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "liquidity", abi = "liquidity()")]
    pub struct LiquidityCall;
    #[doc = "Container type for all input parameters for the `maxLiquidityPerTick`function with signature `maxLiquidityPerTick()` and selector `[112, 207, 117, 74]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "maxLiquidityPerTick", abi = "maxLiquidityPerTick()")]
    pub struct MaxLiquidityPerTickCall;
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address,int24,int24,uint128,bytes)` and selector `[60, 138, 125, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "mint", abi = "mint(address,int24,int24,uint128,bytes)")]
    pub struct MintCall {
        pub recipient: ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `observations`function with signature `observations(uint256)` and selector `[37, 44, 9, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "observations", abi = "observations(uint256)")]
    pub struct ObservationsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `observe`function with signature `observe(uint32[])` and selector `[136, 59, 219, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "observe", abi = "observe(uint32[])")]
    pub struct ObserveCall {
        pub seconds_agos: ::std::vec::Vec<u32>,
    }
    #[doc = "Container type for all input parameters for the `positions`function with signature `positions(bytes32)` and selector `[81, 78, 164, 191]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "positions", abi = "positions(bytes32)")]
    pub struct PositionsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `protocolFees`function with signature `protocolFees()` and selector `[26, 216, 176, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "protocolFees", abi = "protocolFees()")]
    pub struct ProtocolFeesCall;
    #[doc = "Container type for all input parameters for the `setFeeProtocol`function with signature `setFeeProtocol(uint8,uint8)` and selector `[130, 6, 164, 209]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setFeeProtocol", abi = "setFeeProtocol(uint8,uint8)")]
    pub struct SetFeeProtocolCall {
        pub fee_protocol_0: u8,
        pub fee_protocol_1: u8,
    }
    #[doc = "Container type for all input parameters for the `slot0`function with signature `slot0()` and selector `[56, 80, 199, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "slot0", abi = "slot0()")]
    pub struct Slot0Call;
    #[doc = "Container type for all input parameters for the `snapshotCumulativesInside`function with signature `snapshotCumulativesInside(int24,int24)` and selector `[163, 136, 7, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "snapshotCumulativesInside",
        abi = "snapshotCumulativesInside(int24,int24)"
    )]
    pub struct SnapshotCumulativesInsideCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `swap`function with signature `swap(address,bool,int256,uint160,bytes)` and selector `[18, 138, 203, 8]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "swap", abi = "swap(address,bool,int256,uint160,bytes)")]
    pub struct SwapCall {
        pub recipient: ethers::core::types::Address,
        pub zero_for_one: bool,
        pub amount_specified: I256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `tickBitmap`function with signature `tickBitmap(int16)` and selector `[83, 57, 194, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "tickBitmap", abi = "tickBitmap(int16)")]
    pub struct TickBitmapCall(pub i16);
    #[doc = "Container type for all input parameters for the `tickSpacing`function with signature `tickSpacing()` and selector `[208, 201, 58, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "tickSpacing", abi = "tickSpacing()")]
    pub struct TickSpacingCall;
    #[doc = "Container type for all input parameters for the `ticks`function with signature `ticks(int24)` and selector `[243, 13, 186, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "ticks", abi = "ticks(int24)")]
    pub struct TicksCall(pub i32);
    #[doc = "Container type for all input parameters for the `token0`function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    #[doc = "Container type for all input parameters for the `token1`function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV3Calls {
        Burn(BurnCall),
        Collect(CollectCall),
        CollectProtocol(CollectProtocolCall),
        Factory(FactoryCall),
        Fee(FeeCall),
        FeeGrowthGlobal0X128(FeeGrowthGlobal0X128Call),
        FeeGrowthGlobal1X128(FeeGrowthGlobal1X128Call),
        Flash(FlashCall),
        IncreaseObservationCardinalityNext(IncreaseObservationCardinalityNextCall),
        Initialize(InitializeCall),
        Liquidity(LiquidityCall),
        MaxLiquidityPerTick(MaxLiquidityPerTickCall),
        Mint(MintCall),
        Observations(ObservationsCall),
        Observe(ObserveCall),
        Positions(PositionsCall),
        ProtocolFees(ProtocolFeesCall),
        SetFeeProtocol(SetFeeProtocolCall),
        Slot0(Slot0Call),
        SnapshotCumulativesInside(SnapshotCumulativesInsideCall),
        Swap(SwapCall),
        TickBitmap(TickBitmapCall),
        TickSpacing(TickSpacingCall),
        Ticks(TicksCall),
        Token0(Token0Call),
        Token1(Token1Call),
    }
    impl ethers::core::abi::AbiDecode for UniswapV3Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV3Calls::Burn(decoded));
            }
            if let Ok(decoded) =
                <CollectCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Collect(decoded));
            }
            if let Ok(decoded) =
                <CollectProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::CollectProtocol(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Factory(decoded));
            }
            if let Ok(decoded) = <FeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV3Calls::Fee(decoded));
            }
            if let Ok(decoded) =
                <FeeGrowthGlobal0X128Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::FeeGrowthGlobal0X128(decoded));
            }
            if let Ok(decoded) =
                <FeeGrowthGlobal1X128Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::FeeGrowthGlobal1X128(decoded));
            }
            if let Ok(decoded) = <FlashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Flash(decoded));
            }
            if let Ok(decoded) =
                <IncreaseObservationCardinalityNextCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV3Calls::IncreaseObservationCardinalityNext(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Liquidity(decoded));
            }
            if let Ok(decoded) =
                <MaxLiquidityPerTickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::MaxLiquidityPerTick(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV3Calls::Mint(decoded));
            }
            if let Ok(decoded) =
                <ObservationsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Observations(decoded));
            }
            if let Ok(decoded) =
                <ObserveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Observe(decoded));
            }
            if let Ok(decoded) =
                <PositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Positions(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::ProtocolFees(decoded));
            }
            if let Ok(decoded) =
                <SetFeeProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::SetFeeProtocol(decoded));
            }
            if let Ok(decoded) = <Slot0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Slot0(decoded));
            }
            if let Ok(decoded) =
                <SnapshotCumulativesInsideCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV3Calls::SnapshotCumulativesInside(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV3Calls::Swap(decoded));
            }
            if let Ok(decoded) =
                <TickBitmapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::TickBitmap(decoded));
            }
            if let Ok(decoded) =
                <TickSpacingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::TickSpacing(decoded));
            }
            if let Ok(decoded) = <TicksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Ticks(decoded));
            }
            if let Ok(decoded) = <Token0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Token0(decoded));
            }
            if let Ok(decoded) = <Token1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Token1(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapV3Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapV3Calls::Burn(element) => element.encode(),
                UniswapV3Calls::Collect(element) => element.encode(),
                UniswapV3Calls::CollectProtocol(element) => element.encode(),
                UniswapV3Calls::Factory(element) => element.encode(),
                UniswapV3Calls::Fee(element) => element.encode(),
                UniswapV3Calls::FeeGrowthGlobal0X128(element) => element.encode(),
                UniswapV3Calls::FeeGrowthGlobal1X128(element) => element.encode(),
                UniswapV3Calls::Flash(element) => element.encode(),
                UniswapV3Calls::IncreaseObservationCardinalityNext(element) => element.encode(),
                UniswapV3Calls::Initialize(element) => element.encode(),
                UniswapV3Calls::Liquidity(element) => element.encode(),
                UniswapV3Calls::MaxLiquidityPerTick(element) => element.encode(),
                UniswapV3Calls::Mint(element) => element.encode(),
                UniswapV3Calls::Observations(element) => element.encode(),
                UniswapV3Calls::Observe(element) => element.encode(),
                UniswapV3Calls::Positions(element) => element.encode(),
                UniswapV3Calls::ProtocolFees(element) => element.encode(),
                UniswapV3Calls::SetFeeProtocol(element) => element.encode(),
                UniswapV3Calls::Slot0(element) => element.encode(),
                UniswapV3Calls::SnapshotCumulativesInside(element) => element.encode(),
                UniswapV3Calls::Swap(element) => element.encode(),
                UniswapV3Calls::TickBitmap(element) => element.encode(),
                UniswapV3Calls::TickSpacing(element) => element.encode(),
                UniswapV3Calls::Ticks(element) => element.encode(),
                UniswapV3Calls::Token0(element) => element.encode(),
                UniswapV3Calls::Token1(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapV3Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3Calls::Burn(element) => element.fmt(f),
                UniswapV3Calls::Collect(element) => element.fmt(f),
                UniswapV3Calls::CollectProtocol(element) => element.fmt(f),
                UniswapV3Calls::Factory(element) => element.fmt(f),
                UniswapV3Calls::Fee(element) => element.fmt(f),
                UniswapV3Calls::FeeGrowthGlobal0X128(element) => element.fmt(f),
                UniswapV3Calls::FeeGrowthGlobal1X128(element) => element.fmt(f),
                UniswapV3Calls::Flash(element) => element.fmt(f),
                UniswapV3Calls::IncreaseObservationCardinalityNext(element) => element.fmt(f),
                UniswapV3Calls::Initialize(element) => element.fmt(f),
                UniswapV3Calls::Liquidity(element) => element.fmt(f),
                UniswapV3Calls::MaxLiquidityPerTick(element) => element.fmt(f),
                UniswapV3Calls::Mint(element) => element.fmt(f),
                UniswapV3Calls::Observations(element) => element.fmt(f),
                UniswapV3Calls::Observe(element) => element.fmt(f),
                UniswapV3Calls::Positions(element) => element.fmt(f),
                UniswapV3Calls::ProtocolFees(element) => element.fmt(f),
                UniswapV3Calls::SetFeeProtocol(element) => element.fmt(f),
                UniswapV3Calls::Slot0(element) => element.fmt(f),
                UniswapV3Calls::SnapshotCumulativesInside(element) => element.fmt(f),
                UniswapV3Calls::Swap(element) => element.fmt(f),
                UniswapV3Calls::TickBitmap(element) => element.fmt(f),
                UniswapV3Calls::TickSpacing(element) => element.fmt(f),
                UniswapV3Calls::Ticks(element) => element.fmt(f),
                UniswapV3Calls::Token0(element) => element.fmt(f),
                UniswapV3Calls::Token1(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BurnCall> for UniswapV3Calls {
        fn from(var: BurnCall) -> Self {
            UniswapV3Calls::Burn(var)
        }
    }
    impl ::std::convert::From<CollectCall> for UniswapV3Calls {
        fn from(var: CollectCall) -> Self {
            UniswapV3Calls::Collect(var)
        }
    }
    impl ::std::convert::From<CollectProtocolCall> for UniswapV3Calls {
        fn from(var: CollectProtocolCall) -> Self {
            UniswapV3Calls::CollectProtocol(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for UniswapV3Calls {
        fn from(var: FactoryCall) -> Self {
            UniswapV3Calls::Factory(var)
        }
    }
    impl ::std::convert::From<FeeCall> for UniswapV3Calls {
        fn from(var: FeeCall) -> Self {
            UniswapV3Calls::Fee(var)
        }
    }
    impl ::std::convert::From<FeeGrowthGlobal0X128Call> for UniswapV3Calls {
        fn from(var: FeeGrowthGlobal0X128Call) -> Self {
            UniswapV3Calls::FeeGrowthGlobal0X128(var)
        }
    }
    impl ::std::convert::From<FeeGrowthGlobal1X128Call> for UniswapV3Calls {
        fn from(var: FeeGrowthGlobal1X128Call) -> Self {
            UniswapV3Calls::FeeGrowthGlobal1X128(var)
        }
    }
    impl ::std::convert::From<FlashCall> for UniswapV3Calls {
        fn from(var: FlashCall) -> Self {
            UniswapV3Calls::Flash(var)
        }
    }
    impl ::std::convert::From<IncreaseObservationCardinalityNextCall> for UniswapV3Calls {
        fn from(var: IncreaseObservationCardinalityNextCall) -> Self {
            UniswapV3Calls::IncreaseObservationCardinalityNext(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for UniswapV3Calls {
        fn from(var: InitializeCall) -> Self {
            UniswapV3Calls::Initialize(var)
        }
    }
    impl ::std::convert::From<LiquidityCall> for UniswapV3Calls {
        fn from(var: LiquidityCall) -> Self {
            UniswapV3Calls::Liquidity(var)
        }
    }
    impl ::std::convert::From<MaxLiquidityPerTickCall> for UniswapV3Calls {
        fn from(var: MaxLiquidityPerTickCall) -> Self {
            UniswapV3Calls::MaxLiquidityPerTick(var)
        }
    }
    impl ::std::convert::From<MintCall> for UniswapV3Calls {
        fn from(var: MintCall) -> Self {
            UniswapV3Calls::Mint(var)
        }
    }
    impl ::std::convert::From<ObservationsCall> for UniswapV3Calls {
        fn from(var: ObservationsCall) -> Self {
            UniswapV3Calls::Observations(var)
        }
    }
    impl ::std::convert::From<ObserveCall> for UniswapV3Calls {
        fn from(var: ObserveCall) -> Self {
            UniswapV3Calls::Observe(var)
        }
    }
    impl ::std::convert::From<PositionsCall> for UniswapV3Calls {
        fn from(var: PositionsCall) -> Self {
            UniswapV3Calls::Positions(var)
        }
    }
    impl ::std::convert::From<ProtocolFeesCall> for UniswapV3Calls {
        fn from(var: ProtocolFeesCall) -> Self {
            UniswapV3Calls::ProtocolFees(var)
        }
    }
    impl ::std::convert::From<SetFeeProtocolCall> for UniswapV3Calls {
        fn from(var: SetFeeProtocolCall) -> Self {
            UniswapV3Calls::SetFeeProtocol(var)
        }
    }
    impl ::std::convert::From<Slot0Call> for UniswapV3Calls {
        fn from(var: Slot0Call) -> Self {
            UniswapV3Calls::Slot0(var)
        }
    }
    impl ::std::convert::From<SnapshotCumulativesInsideCall> for UniswapV3Calls {
        fn from(var: SnapshotCumulativesInsideCall) -> Self {
            UniswapV3Calls::SnapshotCumulativesInside(var)
        }
    }
    impl ::std::convert::From<SwapCall> for UniswapV3Calls {
        fn from(var: SwapCall) -> Self {
            UniswapV3Calls::Swap(var)
        }
    }
    impl ::std::convert::From<TickBitmapCall> for UniswapV3Calls {
        fn from(var: TickBitmapCall) -> Self {
            UniswapV3Calls::TickBitmap(var)
        }
    }
    impl ::std::convert::From<TickSpacingCall> for UniswapV3Calls {
        fn from(var: TickSpacingCall) -> Self {
            UniswapV3Calls::TickSpacing(var)
        }
    }
    impl ::std::convert::From<TicksCall> for UniswapV3Calls {
        fn from(var: TicksCall) -> Self {
            UniswapV3Calls::Ticks(var)
        }
    }
    impl ::std::convert::From<Token0Call> for UniswapV3Calls {
        fn from(var: Token0Call) -> Self {
            UniswapV3Calls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for UniswapV3Calls {
        fn from(var: Token1Call) -> Self {
            UniswapV3Calls::Token1(var)
        }
    }
}
