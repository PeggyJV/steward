pub use cellarv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod cellarv1_mod {
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
    #[doc = "CellarV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CELLARV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
        });
    #[derive(Clone)]
    pub struct CellarV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CellarV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CellarV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CellarV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CellarV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), CELLARV1_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAXIMUM_SHARE_LOCK_PERIOD` (0x0402ab63) function"]
        pub fn maximum_share_lock_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([4, 2, 171, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_FEE_CUT` (0xeef33eca) function"]
        pub fn max_fee_cut(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([238, 243, 62, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_PERFORMANCE_FEE` (0xbdca9165) function"]
        pub fn max_performance_fee(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([189, 202, 145, 101], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_PLATFORM_FEE` (0x3998a681) function"]
        pub fn max_platform_fee(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([57, 152, 166, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_POSITIONS` (0xf7b24e08) function"]
        pub fn max_positions(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([247, 178, 78, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_REBALANCE_DEVIATION` (0x6ff1c02a) function"]
        pub fn max_rebalance_deviation(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([111, 241, 192, 42], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MINIMUM_SHARE_LOCK_PERIOD` (0x0051a3b7) function"]
        pub fn minimum_share_lock_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([0, 81, 163, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PRICE_ROUTER_REGISTRY_SLOT` (0x5a400d25) function"]
        pub fn price_router_registry_slot(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([90, 64, 13, 37], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addPosition` (0x4eca8a83) function"]
        pub fn add_position(
            &self,
            index: ethers::core::types::U256,
            position: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 202, 138, 131], (index, position))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedRebalanceDeviation` (0xc244245a) function"]
        pub fn allowed_rebalance_deviation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([194, 68, 36, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `asset` (0x38d52e0f) function"]
        pub fn asset(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convertToAssets` (0x07a2d13a) function"]
        pub fn convert_to_assets(
            &self,
            shares: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([7, 162, 209, 58], shares)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convertToShares` (0xc6e6f592) function"]
        pub fn convert_to_shares(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 230, 245, 146], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x6e553f65) function"]
        pub fn deposit(
            &self,
            assets: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([110, 85, 63, 101], (assets, receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositLimit` (0xecf70858) function"]
        pub fn deposit_limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([236, 247, 8, 88], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeData` (0xe753e600) function"]
        pub fn fee_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                u64,
                u64,
                u64,
                u64,
                [u8; 32],
                ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([231, 83, 230, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPositionType` (0x9e35c65b) function"]
        pub fn get_position_type(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([158, 53, 198, 91], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPositions` (0x80275860) function"]
        pub fn get_positions(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([128, 39, 88, 96], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `holdingPosition` (0x9c5f00c2) function"]
        pub fn holding_position(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([156, 95, 0, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initiateShutdown` (0x0a680e18) function"]
        pub fn initiate_shutdown(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 104, 14, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPositionUsed` (0x472090fe) function"]
        pub fn is_position_used(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([71, 32, 144, 254], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isShutdown` (0xbf86d690) function"]
        pub fn is_shutdown(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 134, 214, 144], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isTrusted` (0x96d64879) function"]
        pub fn is_trusted(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([150, 214, 72, 121], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastAccrual` (0x7b3baab4) function"]
        pub fn last_accrual(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([123, 59, 170, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liftShutdown` (0x5e2c576e) function"]
        pub fn lift_shutdown(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 44, 87, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidityLimit` (0x72163715) function"]
        pub fn liquidity_limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([114, 22, 55, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxDeposit` (0x402d267d) function"]
        pub fn max_deposit(
            &self,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([64, 45, 38, 125], receiver)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxMint` (0xc63d75b6) function"]
        pub fn max_mint(
            &self,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 61, 117, 182], receiver)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxRedeem` (0xd905777e) function"]
        pub fn max_redeem(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([217, 5, 119, 126], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxWithdraw` (0xce96cb77) function"]
        pub fn max_withdraw(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([206, 150, 203, 119], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x94bf804d) function"]
        pub fn mint(
            &self,
            shares: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([148, 191, 128, 77], (shares, receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `positions` (0x99fbab88) function"]
        pub fn positions(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([153, 251, 171, 136], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `previewDeposit` (0xef8b30f7) function"]
        pub fn preview_deposit(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([239, 139, 48, 247], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `previewMint` (0xb3d7f6b9) function"]
        pub fn preview_mint(
            &self,
            shares: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([179, 215, 246, 185], shares)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `previewRedeem` (0x4cdad506) function"]
        pub fn preview_redeem(
            &self,
            shares: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([76, 218, 213, 6], shares)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `previewWithdraw` (0x0a28a477) function"]
        pub fn preview_withdraw(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([10, 40, 164, 119], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pushPosition` (0xfdd230b9) function"]
        pub fn push_position(
            &self,
            position: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 210, 48, 185], position)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebalance` (0x389a7294) function"]
        pub fn rebalance(
            &self,
            from_position: ethers::core::types::Address,
            to_position: ethers::core::types::Address,
            assets_from: ethers::core::types::U256,
            exchange: u8,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [56, 154, 114, 148],
                    (from_position, to_position, assets_from, exchange, params),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeem` (0xba087652) function"]
        pub fn redeem(
            &self,
            shares: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([186, 8, 118, 82], (shares, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registry` (0x7b103999) function"]
        pub fn registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([123, 16, 57, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removePosition` (0xc0467422) function"]
        pub fn remove_position(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 70, 116, 34], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `resetHighWatermark` (0xc85e5e13) function"]
        pub fn reset_high_watermark(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 94, 94, 19], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sendFees` (0xdff90b5b) function"]
        pub fn send_fees(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 249, 11, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDepositLimit` (0xbdc8144b) function"]
        pub fn set_deposit_limit(
            &self,
            new_limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 200, 20, 75], new_limit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeesDistributor` (0x6e85f183) function"]
        pub fn set_fees_distributor(
            &self,
            new_fees_distributor: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 133, 241, 131], new_fees_distributor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHoldingPosition` (0x8b0cebf7) function"]
        pub fn set_holding_position(
            &self,
            new_holding_position: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 12, 235, 247], new_holding_position)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidityLimit` (0xdf05a52a) function"]
        pub fn set_liquidity_limit(
            &self,
            new_limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 5, 165, 42], new_limit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOwner` (0x13af4035) function"]
        pub fn set_owner(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPerformanceFee` (0x3cf99a46) function"]
        pub fn set_performance_fee(
            &self,
            new_performance_fee: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 249, 154, 70], new_performance_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPlatformFee` (0x70af7df6) function"]
        pub fn set_platform_fee(
            &self,
            new_platform_fee: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 175, 125, 246], new_platform_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRebalanceDeviation` (0x530a3714) function"]
        pub fn set_rebalance_deviation(
            &self,
            new_deviation: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 10, 55, 20], new_deviation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setShareLockPeriod` (0x9c552ca8) function"]
        pub fn set_share_lock_period(
            &self,
            new_lock: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 85, 44, 168], new_lock)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStrategistPayoutAddress` (0xb0a75d36) function"]
        pub fn set_strategist_payout_address(
            &self,
            payout: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 167, 93, 54], payout)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStrategistPerformanceCut` (0x9b6fd18e) function"]
        pub fn set_strategist_performance_cut(
            &self,
            cut: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 111, 209, 142], cut)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStrategistPlatformCut` (0xb5292a99) function"]
        pub fn set_strategist_platform_cut(
            &self,
            cut: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 41, 42, 153], cut)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setWithdrawType` (0x2f3b5a13) function"]
        pub fn set_withdraw_type(
            &self,
            new_withdraw_type: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 59, 90, 19], new_withdraw_type)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `shareLockPeriod` (0x9fdb11b6) function"]
        pub fn share_lock_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([159, 219, 17, 182], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapPositions` (0x58384573) function"]
        pub fn swap_positions(
            &self,
            index_1: ethers::core::types::U256,
            index_2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 56, 69, 115], (index_1, index_2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalAssets` (0x01e1d114) function"]
        pub fn total_assets(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([1, 225, 209, 20], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalAssetsWithdrawable` (0xa8144e48) function"]
        pub fn total_assets_withdrawable(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([168, 20, 78, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustPosition` (0xfc4d43be) function"]
        pub fn trust_position(
            &self,
            position: ethers::core::types::Address,
            position_type: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 77, 67, 190], (position, position_type))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `userShareLockStartBlock` (0xfc444591) function"]
        pub fn user_share_lock_start_block(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([252, 68, 69, 145], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xb460af94) function"]
        pub fn withdraw(
            &self,
            assets: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([180, 96, 175, 148], (assets, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawType` (0xe39448e0) function"]
        pub fn withdraw_type(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([227, 148, 72, 224], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DepositLimitChanged` event"]
        pub fn deposit_limit_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositLimitChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FeesDistributorChanged` event"]
        pub fn fees_distributor_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FeesDistributorChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `HighWatermarkReset` event"]
        pub fn high_watermark_reset_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, HighWatermarkResetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `HoldingPositionChanged` event"]
        pub fn holding_position_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, HoldingPositionChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidityLimitChanged` event"]
        pub fn liquidity_limit_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidityLimitChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnerUpdated` event"]
        pub fn owner_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PerformanceFeeChanged` event"]
        pub fn performance_fee_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PerformanceFeeChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PlatformFeeChanged` event"]
        pub fn platform_fee_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PlatformFeeChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionAdded` event"]
        pub fn position_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionRemoved` event"]
        pub fn position_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionRemovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionSwapped` event"]
        pub fn position_swapped_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionSwappedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PulledFromPosition` event"]
        pub fn pulled_from_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PulledFromPositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Rebalance` event"]
        pub fn rebalance_filter(&self) -> ethers::contract::builders::Event<M, RebalanceFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RebalanceDeviationChanged` event"]
        pub fn rebalance_deviation_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RebalanceDeviationChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SendFees` event"]
        pub fn send_fees_filter(&self) -> ethers::contract::builders::Event<M, SendFeesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ShareLockingPeriodChanged` event"]
        pub fn share_locking_period_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ShareLockingPeriodChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ShutdownChanged` event"]
        pub fn shutdown_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ShutdownChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StrategistPayoutAddressChanged` event"]
        pub fn strategist_payout_address_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StrategistPayoutAddressChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StrategistPerformanceCutChanged` event"]
        pub fn strategist_performance_cut_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StrategistPerformanceCutChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StrategistPlatformCutChanged` event"]
        pub fn strategist_platform_cut_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StrategistPlatformCutChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TrustChanged` event"]
        pub fn trust_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TrustChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdraw` event"]
        pub fn withdraw_filter(&self) -> ethers::contract::builders::Event<M, WithdrawFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `WithdrawTypeChanged` event"]
        pub fn withdraw_type_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WithdrawTypeChangedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CellarV1Events> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,uint256,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub caller: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
        pub shares: ethers::core::types::U256,
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
        name = "DepositLimitChanged",
        abi = "DepositLimitChanged(uint256,uint256)"
    )]
    pub struct DepositLimitChangedFilter {
        pub old_limit: ethers::core::types::U256,
        pub new_limit: ethers::core::types::U256,
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
        name = "FeesDistributorChanged",
        abi = "FeesDistributorChanged(bytes32,bytes32)"
    )]
    pub struct FeesDistributorChangedFilter {
        pub old_fees_distributor: [u8; 32],
        pub new_fees_distributor: [u8; 32],
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
    #[ethevent(name = "HighWatermarkReset", abi = "HighWatermarkReset(uint256)")]
    pub struct HighWatermarkResetFilter {
        pub new_high_watermark: ethers::core::types::U256,
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
        name = "HoldingPositionChanged",
        abi = "HoldingPositionChanged(address,address)"
    )]
    pub struct HoldingPositionChangedFilter {
        #[ethevent(indexed)]
        pub old_position: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_position: ethers::core::types::Address,
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
        name = "LiquidityLimitChanged",
        abi = "LiquidityLimitChanged(uint256,uint256)"
    )]
    pub struct LiquidityLimitChangedFilter {
        pub old_limit: ethers::core::types::U256,
        pub new_limit: ethers::core::types::U256,
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
    #[ethevent(name = "OwnerUpdated", abi = "OwnerUpdated(address,address)")]
    pub struct OwnerUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
        name = "PerformanceFeeChanged",
        abi = "PerformanceFeeChanged(uint64,uint64)"
    )]
    pub struct PerformanceFeeChangedFilter {
        pub old_performance_fee: u64,
        pub new_performance_fee: u64,
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
    #[ethevent(name = "PlatformFeeChanged", abi = "PlatformFeeChanged(uint64,uint64)")]
    pub struct PlatformFeeChangedFilter {
        pub old_platform_fee: u64,
        pub new_platform_fee: u64,
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
    #[ethevent(name = "PositionAdded", abi = "PositionAdded(address,uint256)")]
    pub struct PositionAddedFilter {
        #[ethevent(indexed)]
        pub position: ethers::core::types::Address,
        pub index: ethers::core::types::U256,
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
    #[ethevent(name = "PositionRemoved", abi = "PositionRemoved(address,uint256)")]
    pub struct PositionRemovedFilter {
        #[ethevent(indexed)]
        pub position: ethers::core::types::Address,
        pub index: ethers::core::types::U256,
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
        name = "PositionSwapped",
        abi = "PositionSwapped(address,address,uint256,uint256)"
    )]
    pub struct PositionSwappedFilter {
        #[ethevent(indexed)]
        pub new_position_1: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_position_2: ethers::core::types::Address,
        pub index_1: ethers::core::types::U256,
        pub index_2: ethers::core::types::U256,
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
        name = "PulledFromPosition",
        abi = "PulledFromPosition(address,uint256)"
    )]
    pub struct PulledFromPositionFilter {
        #[ethevent(indexed)]
        pub position: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "Rebalance", abi = "Rebalance(address,address,uint256,uint256)")]
    pub struct RebalanceFilter {
        #[ethevent(indexed)]
        pub from_position: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_position: ethers::core::types::Address,
        pub assets_from: ethers::core::types::U256,
        pub assets_to: ethers::core::types::U256,
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
        name = "RebalanceDeviationChanged",
        abi = "RebalanceDeviationChanged(uint256,uint256)"
    )]
    pub struct RebalanceDeviationChangedFilter {
        pub old_deviation: ethers::core::types::U256,
        pub new_deviation: ethers::core::types::U256,
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
    #[ethevent(name = "SendFees", abi = "SendFees(uint256,uint256)")]
    pub struct SendFeesFilter {
        pub fees_in_shares_redeemed: ethers::core::types::U256,
        pub fees_in_assets_sent: ethers::core::types::U256,
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
        name = "ShareLockingPeriodChanged",
        abi = "ShareLockingPeriodChanged(uint256,uint256)"
    )]
    pub struct ShareLockingPeriodChangedFilter {
        pub old_period: ethers::core::types::U256,
        pub new_period: ethers::core::types::U256,
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
    #[ethevent(name = "ShutdownChanged", abi = "ShutdownChanged(bool)")]
    pub struct ShutdownChangedFilter {
        pub is_shutdown: bool,
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
        name = "StrategistPayoutAddressChanged",
        abi = "StrategistPayoutAddressChanged(address,address)"
    )]
    pub struct StrategistPayoutAddressChangedFilter {
        pub old_payout_address: ethers::core::types::Address,
        pub new_payout_address: ethers::core::types::Address,
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
        name = "StrategistPerformanceCutChanged",
        abi = "StrategistPerformanceCutChanged(uint64,uint64)"
    )]
    pub struct StrategistPerformanceCutChangedFilter {
        pub old_performance_cut: u64,
        pub new_performance_cut: u64,
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
        name = "StrategistPlatformCutChanged",
        abi = "StrategistPlatformCutChanged(uint64,uint64)"
    )]
    pub struct StrategistPlatformCutChangedFilter {
        pub old_platform_cut: u64,
        pub new_platform_cut: u64,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "TrustChanged", abi = "TrustChanged(address,bool)")]
    pub struct TrustChangedFilter {
        #[ethevent(indexed)]
        pub position: ethers::core::types::Address,
        pub is_trusted: bool,
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
        name = "Withdraw",
        abi = "Withdraw(address,address,address,uint256,uint256)"
    )]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub caller: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
        pub shares: ethers::core::types::U256,
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
    #[ethevent(name = "WithdrawTypeChanged", abi = "WithdrawTypeChanged(uint8,uint8)")]
    pub struct WithdrawTypeChangedFilter {
        pub old_type: u8,
        pub new_type: u8,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CellarV1Events {
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        DepositLimitChangedFilter(DepositLimitChangedFilter),
        FeesDistributorChangedFilter(FeesDistributorChangedFilter),
        HighWatermarkResetFilter(HighWatermarkResetFilter),
        HoldingPositionChangedFilter(HoldingPositionChangedFilter),
        LiquidityLimitChangedFilter(LiquidityLimitChangedFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        PerformanceFeeChangedFilter(PerformanceFeeChangedFilter),
        PlatformFeeChangedFilter(PlatformFeeChangedFilter),
        PositionAddedFilter(PositionAddedFilter),
        PositionRemovedFilter(PositionRemovedFilter),
        PositionSwappedFilter(PositionSwappedFilter),
        PulledFromPositionFilter(PulledFromPositionFilter),
        RebalanceFilter(RebalanceFilter),
        RebalanceDeviationChangedFilter(RebalanceDeviationChangedFilter),
        SendFeesFilter(SendFeesFilter),
        ShareLockingPeriodChangedFilter(ShareLockingPeriodChangedFilter),
        ShutdownChangedFilter(ShutdownChangedFilter),
        StrategistPayoutAddressChangedFilter(StrategistPayoutAddressChangedFilter),
        StrategistPerformanceCutChangedFilter(StrategistPerformanceCutChangedFilter),
        StrategistPlatformCutChangedFilter(StrategistPlatformCutChangedFilter),
        TransferFilter(TransferFilter),
        TrustChangedFilter(TrustChangedFilter),
        WithdrawFilter(WithdrawFilter),
        WithdrawTypeChangedFilter(WithdrawTypeChangedFilter),
    }
    impl ethers::contract::EthLogDecode for CellarV1Events {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CellarV1Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(CellarV1Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = DepositLimitChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::DepositLimitChangedFilter(decoded));
            }
            if let Ok(decoded) = FeesDistributorChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::FeesDistributorChangedFilter(decoded));
            }
            if let Ok(decoded) = HighWatermarkResetFilter::decode_log(log) {
                return Ok(CellarV1Events::HighWatermarkResetFilter(decoded));
            }
            if let Ok(decoded) = HoldingPositionChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::HoldingPositionChangedFilter(decoded));
            }
            if let Ok(decoded) = LiquidityLimitChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::LiquidityLimitChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(CellarV1Events::OwnerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PerformanceFeeChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::PerformanceFeeChangedFilter(decoded));
            }
            if let Ok(decoded) = PlatformFeeChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::PlatformFeeChangedFilter(decoded));
            }
            if let Ok(decoded) = PositionAddedFilter::decode_log(log) {
                return Ok(CellarV1Events::PositionAddedFilter(decoded));
            }
            if let Ok(decoded) = PositionRemovedFilter::decode_log(log) {
                return Ok(CellarV1Events::PositionRemovedFilter(decoded));
            }
            if let Ok(decoded) = PositionSwappedFilter::decode_log(log) {
                return Ok(CellarV1Events::PositionSwappedFilter(decoded));
            }
            if let Ok(decoded) = PulledFromPositionFilter::decode_log(log) {
                return Ok(CellarV1Events::PulledFromPositionFilter(decoded));
            }
            if let Ok(decoded) = RebalanceFilter::decode_log(log) {
                return Ok(CellarV1Events::RebalanceFilter(decoded));
            }
            if let Ok(decoded) = RebalanceDeviationChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::RebalanceDeviationChangedFilter(decoded));
            }
            if let Ok(decoded) = SendFeesFilter::decode_log(log) {
                return Ok(CellarV1Events::SendFeesFilter(decoded));
            }
            if let Ok(decoded) = ShareLockingPeriodChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::ShareLockingPeriodChangedFilter(decoded));
            }
            if let Ok(decoded) = ShutdownChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::ShutdownChangedFilter(decoded));
            }
            if let Ok(decoded) = StrategistPayoutAddressChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::StrategistPayoutAddressChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StrategistPerformanceCutChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::StrategistPerformanceCutChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StrategistPlatformCutChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::StrategistPlatformCutChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CellarV1Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = TrustChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::TrustChangedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(CellarV1Events::WithdrawFilter(decoded));
            }
            if let Ok(decoded) = WithdrawTypeChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::WithdrawTypeChangedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CellarV1Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarV1Events::ApprovalFilter(element) => element.fmt(f),
                CellarV1Events::DepositFilter(element) => element.fmt(f),
                CellarV1Events::DepositLimitChangedFilter(element) => element.fmt(f),
                CellarV1Events::FeesDistributorChangedFilter(element) => element.fmt(f),
                CellarV1Events::HighWatermarkResetFilter(element) => element.fmt(f),
                CellarV1Events::HoldingPositionChangedFilter(element) => element.fmt(f),
                CellarV1Events::LiquidityLimitChangedFilter(element) => element.fmt(f),
                CellarV1Events::OwnerUpdatedFilter(element) => element.fmt(f),
                CellarV1Events::PerformanceFeeChangedFilter(element) => element.fmt(f),
                CellarV1Events::PlatformFeeChangedFilter(element) => element.fmt(f),
                CellarV1Events::PositionAddedFilter(element) => element.fmt(f),
                CellarV1Events::PositionRemovedFilter(element) => element.fmt(f),
                CellarV1Events::PositionSwappedFilter(element) => element.fmt(f),
                CellarV1Events::PulledFromPositionFilter(element) => element.fmt(f),
                CellarV1Events::RebalanceFilter(element) => element.fmt(f),
                CellarV1Events::RebalanceDeviationChangedFilter(element) => element.fmt(f),
                CellarV1Events::SendFeesFilter(element) => element.fmt(f),
                CellarV1Events::ShareLockingPeriodChangedFilter(element) => element.fmt(f),
                CellarV1Events::ShutdownChangedFilter(element) => element.fmt(f),
                CellarV1Events::StrategistPayoutAddressChangedFilter(element) => element.fmt(f),
                CellarV1Events::StrategistPerformanceCutChangedFilter(element) => element.fmt(f),
                CellarV1Events::StrategistPlatformCutChangedFilter(element) => element.fmt(f),
                CellarV1Events::TransferFilter(element) => element.fmt(f),
                CellarV1Events::TrustChangedFilter(element) => element.fmt(f),
                CellarV1Events::WithdrawFilter(element) => element.fmt(f),
                CellarV1Events::WithdrawTypeChangedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR`function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `MAXIMUM_SHARE_LOCK_PERIOD`function with signature `MAXIMUM_SHARE_LOCK_PERIOD()` and selector `[4, 2, 171, 99]`"]
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
        name = "MAXIMUM_SHARE_LOCK_PERIOD",
        abi = "MAXIMUM_SHARE_LOCK_PERIOD()"
    )]
    pub struct MaximumShareLockPeriodCall;
    #[doc = "Container type for all input parameters for the `MAX_FEE_CUT`function with signature `MAX_FEE_CUT()` and selector `[238, 243, 62, 202]`"]
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
    #[ethcall(name = "MAX_FEE_CUT", abi = "MAX_FEE_CUT()")]
    pub struct MaxFeeCutCall;
    #[doc = "Container type for all input parameters for the `MAX_PERFORMANCE_FEE`function with signature `MAX_PERFORMANCE_FEE()` and selector `[189, 202, 145, 101]`"]
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
    #[ethcall(name = "MAX_PERFORMANCE_FEE", abi = "MAX_PERFORMANCE_FEE()")]
    pub struct MaxPerformanceFeeCall;
    #[doc = "Container type for all input parameters for the `MAX_PLATFORM_FEE`function with signature `MAX_PLATFORM_FEE()` and selector `[57, 152, 166, 129]`"]
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
    #[ethcall(name = "MAX_PLATFORM_FEE", abi = "MAX_PLATFORM_FEE()")]
    pub struct MaxPlatformFeeCall;
    #[doc = "Container type for all input parameters for the `MAX_POSITIONS`function with signature `MAX_POSITIONS()` and selector `[247, 178, 78, 8]`"]
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
    #[ethcall(name = "MAX_POSITIONS", abi = "MAX_POSITIONS()")]
    pub struct MaxPositionsCall;
    #[doc = "Container type for all input parameters for the `MAX_REBALANCE_DEVIATION`function with signature `MAX_REBALANCE_DEVIATION()` and selector `[111, 241, 192, 42]`"]
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
    #[ethcall(name = "MAX_REBALANCE_DEVIATION", abi = "MAX_REBALANCE_DEVIATION()")]
    pub struct MaxRebalanceDeviationCall;
    #[doc = "Container type for all input parameters for the `MINIMUM_SHARE_LOCK_PERIOD`function with signature `MINIMUM_SHARE_LOCK_PERIOD()` and selector `[0, 81, 163, 183]`"]
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
        name = "MINIMUM_SHARE_LOCK_PERIOD",
        abi = "MINIMUM_SHARE_LOCK_PERIOD()"
    )]
    pub struct MinimumShareLockPeriodCall;
    #[doc = "Container type for all input parameters for the `PRICE_ROUTER_REGISTRY_SLOT`function with signature `PRICE_ROUTER_REGISTRY_SLOT()` and selector `[90, 64, 13, 37]`"]
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
        name = "PRICE_ROUTER_REGISTRY_SLOT",
        abi = "PRICE_ROUTER_REGISTRY_SLOT()"
    )]
    pub struct PriceRouterRegistrySlotCall;
    #[doc = "Container type for all input parameters for the `addPosition`function with signature `addPosition(uint256,address)` and selector `[78, 202, 138, 131]`"]
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
    #[ethcall(name = "addPosition", abi = "addPosition(uint256,address)")]
    pub struct AddPositionCall {
        pub index: ethers::core::types::U256,
        pub position: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `allowedRebalanceDeviation`function with signature `allowedRebalanceDeviation()` and selector `[194, 68, 36, 90]`"]
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
        name = "allowedRebalanceDeviation",
        abi = "allowedRebalanceDeviation()"
    )]
    pub struct AllowedRebalanceDeviationCall;
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `asset`function with signature `asset()` and selector `[56, 213, 46, 15]`"]
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `convertToAssets`function with signature `convertToAssets(uint256)` and selector `[7, 162, 209, 58]`"]
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
    #[ethcall(name = "convertToAssets", abi = "convertToAssets(uint256)")]
    pub struct ConvertToAssetsCall {
        pub shares: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `convertToShares`function with signature `convertToShares(uint256)` and selector `[198, 230, 245, 146]`"]
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
    #[ethcall(name = "convertToShares", abi = "convertToShares(uint256)")]
    pub struct ConvertToSharesCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `decreaseAllowance`function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
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
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256,address)` and selector `[110, 85, 63, 101]`"]
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,address)")]
    pub struct DepositCall {
        pub assets: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `depositLimit`function with signature `depositLimit()` and selector `[236, 247, 8, 88]`"]
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
    #[ethcall(name = "depositLimit", abi = "depositLimit()")]
    pub struct DepositLimitCall;
    #[doc = "Container type for all input parameters for the `feeData`function with signature `feeData()` and selector `[231, 83, 230, 0]`"]
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
    #[ethcall(name = "feeData", abi = "feeData()")]
    pub struct FeeDataCall;
    #[doc = "Container type for all input parameters for the `getPositionType`function with signature `getPositionType(address)` and selector `[158, 53, 198, 91]`"]
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
    #[ethcall(name = "getPositionType", abi = "getPositionType(address)")]
    pub struct GetPositionTypeCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `getPositions`function with signature `getPositions()` and selector `[128, 39, 88, 96]`"]
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
    #[ethcall(name = "getPositions", abi = "getPositions()")]
    pub struct GetPositionsCall;
    #[doc = "Container type for all input parameters for the `holdingPosition`function with signature `holdingPosition()` and selector `[156, 95, 0, 194]`"]
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
    #[ethcall(name = "holdingPosition", abi = "holdingPosition()")]
    pub struct HoldingPositionCall;
    #[doc = "Container type for all input parameters for the `increaseAllowance`function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
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
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `initiateShutdown`function with signature `initiateShutdown()` and selector `[10, 104, 14, 24]`"]
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
    #[ethcall(name = "initiateShutdown", abi = "initiateShutdown()")]
    pub struct InitiateShutdownCall;
    #[doc = "Container type for all input parameters for the `isPositionUsed`function with signature `isPositionUsed(address)` and selector `[71, 32, 144, 254]`"]
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
    #[ethcall(name = "isPositionUsed", abi = "isPositionUsed(address)")]
    pub struct IsPositionUsedCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `isShutdown`function with signature `isShutdown()` and selector `[191, 134, 214, 144]`"]
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
    #[ethcall(name = "isShutdown", abi = "isShutdown()")]
    pub struct IsShutdownCall;
    #[doc = "Container type for all input parameters for the `isTrusted`function with signature `isTrusted(address)` and selector `[150, 214, 72, 121]`"]
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
    #[ethcall(name = "isTrusted", abi = "isTrusted(address)")]
    pub struct IsTrustedCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `lastAccrual`function with signature `lastAccrual()` and selector `[123, 59, 170, 180]`"]
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
    #[ethcall(name = "lastAccrual", abi = "lastAccrual()")]
    pub struct LastAccrualCall;
    #[doc = "Container type for all input parameters for the `liftShutdown`function with signature `liftShutdown()` and selector `[94, 44, 87, 110]`"]
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
    #[ethcall(name = "liftShutdown", abi = "liftShutdown()")]
    pub struct LiftShutdownCall;
    #[doc = "Container type for all input parameters for the `liquidityLimit`function with signature `liquidityLimit()` and selector `[114, 22, 55, 21]`"]
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
    #[ethcall(name = "liquidityLimit", abi = "liquidityLimit()")]
    pub struct LiquidityLimitCall;
    #[doc = "Container type for all input parameters for the `maxDeposit`function with signature `maxDeposit(address)` and selector `[64, 45, 38, 125]`"]
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
    #[ethcall(name = "maxDeposit", abi = "maxDeposit(address)")]
    pub struct MaxDepositCall {
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `maxMint`function with signature `maxMint(address)` and selector `[198, 61, 117, 182]`"]
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
    #[ethcall(name = "maxMint", abi = "maxMint(address)")]
    pub struct MaxMintCall {
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `maxRedeem`function with signature `maxRedeem(address)` and selector `[217, 5, 119, 126]`"]
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
    #[ethcall(name = "maxRedeem", abi = "maxRedeem(address)")]
    pub struct MaxRedeemCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `maxWithdraw`function with signature `maxWithdraw(address)` and selector `[206, 150, 203, 119]`"]
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
    #[ethcall(name = "maxWithdraw", abi = "maxWithdraw(address)")]
    pub struct MaxWithdrawCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(uint256,address)` and selector `[148, 191, 128, 77]`"]
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
    #[ethcall(name = "mint", abi = "mint(uint256,address)")]
    pub struct MintCall {
        pub shares: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `nonces`function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `permit`function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `positions`function with signature `positions(uint256)` and selector `[153, 251, 171, 136]`"]
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
    #[ethcall(name = "positions", abi = "positions(uint256)")]
    pub struct PositionsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `previewDeposit`function with signature `previewDeposit(uint256)` and selector `[239, 139, 48, 247]`"]
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
    #[ethcall(name = "previewDeposit", abi = "previewDeposit(uint256)")]
    pub struct PreviewDepositCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `previewMint`function with signature `previewMint(uint256)` and selector `[179, 215, 246, 185]`"]
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
    #[ethcall(name = "previewMint", abi = "previewMint(uint256)")]
    pub struct PreviewMintCall {
        pub shares: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `previewRedeem`function with signature `previewRedeem(uint256)` and selector `[76, 218, 213, 6]`"]
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
    #[ethcall(name = "previewRedeem", abi = "previewRedeem(uint256)")]
    pub struct PreviewRedeemCall {
        pub shares: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `previewWithdraw`function with signature `previewWithdraw(uint256)` and selector `[10, 40, 164, 119]`"]
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
    #[ethcall(name = "previewWithdraw", abi = "previewWithdraw(uint256)")]
    pub struct PreviewWithdrawCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `pushPosition`function with signature `pushPosition(address)` and selector `[253, 210, 48, 185]`"]
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
    #[ethcall(name = "pushPosition", abi = "pushPosition(address)")]
    pub struct PushPositionCall {
        pub position: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `rebalance`function with signature `rebalance(address,address,uint256,uint8,bytes)` and selector `[56, 154, 114, 148]`"]
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
        name = "rebalance",
        abi = "rebalance(address,address,uint256,uint8,bytes)"
    )]
    pub struct RebalanceCall {
        pub from_position: ethers::core::types::Address,
        pub to_position: ethers::core::types::Address,
        pub assets_from: ethers::core::types::U256,
        pub exchange: u8,
        pub params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `redeem`function with signature `redeem(uint256,address,address)` and selector `[186, 8, 118, 82]`"]
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,address,address)")]
    pub struct RedeemCall {
        pub shares: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `registry`function with signature `registry()` and selector `[123, 16, 57, 153]`"]
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
    #[ethcall(name = "registry", abi = "registry()")]
    pub struct RegistryCall;
    #[doc = "Container type for all input parameters for the `removePosition`function with signature `removePosition(uint256)` and selector `[192, 70, 116, 34]`"]
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
    #[ethcall(name = "removePosition", abi = "removePosition(uint256)")]
    pub struct RemovePositionCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `resetHighWatermark`function with signature `resetHighWatermark()` and selector `[200, 94, 94, 19]`"]
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
    #[ethcall(name = "resetHighWatermark", abi = "resetHighWatermark()")]
    pub struct ResetHighWatermarkCall;
    #[doc = "Container type for all input parameters for the `sendFees`function with signature `sendFees()` and selector `[223, 249, 11, 91]`"]
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
    #[ethcall(name = "sendFees", abi = "sendFees()")]
    pub struct SendFeesCall;
    #[doc = "Container type for all input parameters for the `setDepositLimit`function with signature `setDepositLimit(uint256)` and selector `[189, 200, 20, 75]`"]
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
    #[ethcall(name = "setDepositLimit", abi = "setDepositLimit(uint256)")]
    pub struct SetDepositLimitCall {
        pub new_limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setFeesDistributor`function with signature `setFeesDistributor(bytes32)` and selector `[110, 133, 241, 131]`"]
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
    #[ethcall(name = "setFeesDistributor", abi = "setFeesDistributor(bytes32)")]
    pub struct SetFeesDistributorCall {
        pub new_fees_distributor: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setHoldingPosition`function with signature `setHoldingPosition(address)` and selector `[139, 12, 235, 247]`"]
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
    #[ethcall(name = "setHoldingPosition", abi = "setHoldingPosition(address)")]
    pub struct SetHoldingPositionCall {
        pub new_holding_position: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setLiquidityLimit`function with signature `setLiquidityLimit(uint256)` and selector `[223, 5, 165, 42]`"]
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
    #[ethcall(name = "setLiquidityLimit", abi = "setLiquidityLimit(uint256)")]
    pub struct SetLiquidityLimitCall {
        pub new_limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setOwner`function with signature `setOwner(address)` and selector `[19, 175, 64, 53]`"]
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
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPerformanceFee`function with signature `setPerformanceFee(uint64)` and selector `[60, 249, 154, 70]`"]
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
    #[ethcall(name = "setPerformanceFee", abi = "setPerformanceFee(uint64)")]
    pub struct SetPerformanceFeeCall {
        pub new_performance_fee: u64,
    }
    #[doc = "Container type for all input parameters for the `setPlatformFee`function with signature `setPlatformFee(uint64)` and selector `[112, 175, 125, 246]`"]
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
    #[ethcall(name = "setPlatformFee", abi = "setPlatformFee(uint64)")]
    pub struct SetPlatformFeeCall {
        pub new_platform_fee: u64,
    }
    #[doc = "Container type for all input parameters for the `setRebalanceDeviation`function with signature `setRebalanceDeviation(uint256)` and selector `[83, 10, 55, 20]`"]
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
    #[ethcall(name = "setRebalanceDeviation", abi = "setRebalanceDeviation(uint256)")]
    pub struct SetRebalanceDeviationCall {
        pub new_deviation: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setShareLockPeriod`function with signature `setShareLockPeriod(uint256)` and selector `[156, 85, 44, 168]`"]
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
    #[ethcall(name = "setShareLockPeriod", abi = "setShareLockPeriod(uint256)")]
    pub struct SetShareLockPeriodCall {
        pub new_lock: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setStrategistPayoutAddress`function with signature `setStrategistPayoutAddress(address)` and selector `[176, 167, 93, 54]`"]
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
        name = "setStrategistPayoutAddress",
        abi = "setStrategistPayoutAddress(address)"
    )]
    pub struct SetStrategistPayoutAddressCall {
        pub payout: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setStrategistPerformanceCut`function with signature `setStrategistPerformanceCut(uint64)` and selector `[155, 111, 209, 142]`"]
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
        name = "setStrategistPerformanceCut",
        abi = "setStrategistPerformanceCut(uint64)"
    )]
    pub struct SetStrategistPerformanceCutCall {
        pub cut: u64,
    }
    #[doc = "Container type for all input parameters for the `setStrategistPlatformCut`function with signature `setStrategistPlatformCut(uint64)` and selector `[181, 41, 42, 153]`"]
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
        name = "setStrategistPlatformCut",
        abi = "setStrategistPlatformCut(uint64)"
    )]
    pub struct SetStrategistPlatformCutCall {
        pub cut: u64,
    }
    #[doc = "Container type for all input parameters for the `setWithdrawType`function with signature `setWithdrawType(uint8)` and selector `[47, 59, 90, 19]`"]
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
    #[ethcall(name = "setWithdrawType", abi = "setWithdrawType(uint8)")]
    pub struct SetWithdrawTypeCall {
        pub new_withdraw_type: u8,
    }
    #[doc = "Container type for all input parameters for the `shareLockPeriod`function with signature `shareLockPeriod()` and selector `[159, 219, 17, 182]`"]
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
    #[ethcall(name = "shareLockPeriod", abi = "shareLockPeriod()")]
    pub struct ShareLockPeriodCall;
    #[doc = "Container type for all input parameters for the `swapPositions`function with signature `swapPositions(uint256,uint256)` and selector `[88, 56, 69, 115]`"]
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
    #[ethcall(name = "swapPositions", abi = "swapPositions(uint256,uint256)")]
    pub struct SwapPositionsCall {
        pub index_1: ethers::core::types::U256,
        pub index_2: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalAssets`function with signature `totalAssets()` and selector `[1, 225, 209, 20]`"]
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
    #[ethcall(name = "totalAssets", abi = "totalAssets()")]
    pub struct TotalAssetsCall;
    #[doc = "Container type for all input parameters for the `totalAssetsWithdrawable`function with signature `totalAssetsWithdrawable()` and selector `[168, 20, 78, 72]`"]
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
    #[ethcall(name = "totalAssetsWithdrawable", abi = "totalAssetsWithdrawable()")]
    pub struct TotalAssetsWithdrawableCall;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `trustPosition`function with signature `trustPosition(address,uint8)` and selector `[252, 77, 67, 190]`"]
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
    #[ethcall(name = "trustPosition", abi = "trustPosition(address,uint8)")]
    pub struct TrustPositionCall {
        pub position: ethers::core::types::Address,
        pub position_type: u8,
    }
    #[doc = "Container type for all input parameters for the `userShareLockStartBlock`function with signature `userShareLockStartBlock(address)` and selector `[252, 68, 69, 145]`"]
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
        name = "userShareLockStartBlock",
        abi = "userShareLockStartBlock(address)"
    )]
    pub struct UserShareLockStartBlockCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256,address,address)` and selector `[180, 96, 175, 148]`"]
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,address)")]
    pub struct WithdrawCall {
        pub assets: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdrawType`function with signature `withdrawType()` and selector `[227, 148, 72, 224]`"]
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
    #[ethcall(name = "withdrawType", abi = "withdrawType()")]
    pub struct WithdrawTypeCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CellarV1Calls {
        DomainSeparator(DomainSeparatorCall),
        MaximumShareLockPeriod(MaximumShareLockPeriodCall),
        MaxFeeCut(MaxFeeCutCall),
        MaxPerformanceFee(MaxPerformanceFeeCall),
        MaxPlatformFee(MaxPlatformFeeCall),
        MaxPositions(MaxPositionsCall),
        MaxRebalanceDeviation(MaxRebalanceDeviationCall),
        MinimumShareLockPeriod(MinimumShareLockPeriodCall),
        PriceRouterRegistrySlot(PriceRouterRegistrySlotCall),
        AddPosition(AddPositionCall),
        Allowance(AllowanceCall),
        AllowedRebalanceDeviation(AllowedRebalanceDeviationCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        BalanceOf(BalanceOfCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        Deposit(DepositCall),
        DepositLimit(DepositLimitCall),
        FeeData(FeeDataCall),
        GetPositionType(GetPositionTypeCall),
        GetPositions(GetPositionsCall),
        HoldingPosition(HoldingPositionCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        InitiateShutdown(InitiateShutdownCall),
        IsPositionUsed(IsPositionUsedCall),
        IsShutdown(IsShutdownCall),
        IsTrusted(IsTrustedCall),
        LastAccrual(LastAccrualCall),
        LiftShutdown(LiftShutdownCall),
        LiquidityLimit(LiquidityLimitCall),
        MaxDeposit(MaxDepositCall),
        MaxMint(MaxMintCall),
        MaxRedeem(MaxRedeemCall),
        MaxWithdraw(MaxWithdrawCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        Positions(PositionsCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        PushPosition(PushPositionCall),
        Rebalance(RebalanceCall),
        Redeem(RedeemCall),
        Registry(RegistryCall),
        RemovePosition(RemovePositionCall),
        ResetHighWatermark(ResetHighWatermarkCall),
        SendFees(SendFeesCall),
        SetDepositLimit(SetDepositLimitCall),
        SetFeesDistributor(SetFeesDistributorCall),
        SetHoldingPosition(SetHoldingPositionCall),
        SetLiquidityLimit(SetLiquidityLimitCall),
        SetOwner(SetOwnerCall),
        SetPerformanceFee(SetPerformanceFeeCall),
        SetPlatformFee(SetPlatformFeeCall),
        SetRebalanceDeviation(SetRebalanceDeviationCall),
        SetShareLockPeriod(SetShareLockPeriodCall),
        SetStrategistPayoutAddress(SetStrategistPayoutAddressCall),
        SetStrategistPerformanceCut(SetStrategistPerformanceCutCall),
        SetStrategistPlatformCut(SetStrategistPlatformCutCall),
        SetWithdrawType(SetWithdrawTypeCall),
        ShareLockPeriod(ShareLockPeriodCall),
        SwapPositions(SwapPositionsCall),
        Symbol(SymbolCall),
        TotalAssets(TotalAssetsCall),
        TotalAssetsWithdrawable(TotalAssetsWithdrawableCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TrustPosition(TrustPositionCall),
        UserShareLockStartBlock(UserShareLockStartBlockCall),
        Withdraw(WithdrawCall),
        WithdrawType(WithdrawTypeCall),
    }
    impl ethers::core::abi::AbiDecode for CellarV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <MaximumShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaximumShareLockPeriod(decoded));
            }
            if let Ok(decoded) =
                <MaxFeeCutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaxFeeCut(decoded));
            }
            if let Ok(decoded) =
                <MaxPerformanceFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaxPerformanceFee(decoded));
            }
            if let Ok(decoded) =
                <MaxPlatformFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaxPlatformFee(decoded));
            }
            if let Ok(decoded) =
                <MaxPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaxPositions(decoded));
            }
            if let Ok(decoded) =
                <MaxRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaxRebalanceDeviation(decoded));
            }
            if let Ok(decoded) =
                <MinimumShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MinimumShareLockPeriod(decoded));
            }
            if let Ok(decoded) =
                <PriceRouterRegistrySlotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::PriceRouterRegistrySlot(decoded));
            }
            if let Ok(decoded) =
                <AddPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::AddPosition(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <AllowedRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV1Calls::AllowedRebalanceDeviation(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Approve(decoded));
            }
            if let Ok(decoded) = <AssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Asset(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ConvertToAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::ConvertToAssets(decoded));
            }
            if let Ok(decoded) =
                <ConvertToSharesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::ConvertToShares(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::DepositLimit(decoded));
            }
            if let Ok(decoded) =
                <FeeDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::FeeData(decoded));
            }
            if let Ok(decoded) =
                <GetPositionTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::GetPositionType(decoded));
            }
            if let Ok(decoded) =
                <GetPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::GetPositions(decoded));
            }
            if let Ok(decoded) =
                <HoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::HoldingPosition(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <InitiateShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::InitiateShutdown(decoded));
            }
            if let Ok(decoded) =
                <IsPositionUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::IsPositionUsed(decoded));
            }
            if let Ok(decoded) =
                <IsShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::IsShutdown(decoded));
            }
            if let Ok(decoded) =
                <IsTrustedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::IsTrusted(decoded));
            }
            if let Ok(decoded) =
                <LastAccrualCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::LastAccrual(decoded));
            }
            if let Ok(decoded) =
                <LiftShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::LiftShutdown(decoded));
            }
            if let Ok(decoded) =
                <LiquidityLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::LiquidityLimit(decoded));
            }
            if let Ok(decoded) =
                <MaxDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaxDeposit(decoded));
            }
            if let Ok(decoded) =
                <MaxMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaxMint(decoded));
            }
            if let Ok(decoded) =
                <MaxRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaxRedeem(decoded));
            }
            if let Ok(decoded) =
                <MaxWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::MaxWithdraw(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarV1Calls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarV1Calls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Nonces(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Owner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Permit(decoded));
            }
            if let Ok(decoded) =
                <PositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Positions(decoded));
            }
            if let Ok(decoded) =
                <PreviewDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::PreviewDeposit(decoded));
            }
            if let Ok(decoded) =
                <PreviewMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::PreviewMint(decoded));
            }
            if let Ok(decoded) =
                <PreviewRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::PreviewRedeem(decoded));
            }
            if let Ok(decoded) =
                <PreviewWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::PreviewWithdraw(decoded));
            }
            if let Ok(decoded) =
                <PushPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::PushPosition(decoded));
            }
            if let Ok(decoded) =
                <RebalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Rebalance(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Registry(decoded));
            }
            if let Ok(decoded) =
                <RemovePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::RemovePosition(decoded));
            }
            if let Ok(decoded) =
                <ResetHighWatermarkCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::ResetHighWatermark(decoded));
            }
            if let Ok(decoded) =
                <SendFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SendFees(decoded));
            }
            if let Ok(decoded) =
                <SetDepositLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetDepositLimit(decoded));
            }
            if let Ok(decoded) =
                <SetFeesDistributorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetFeesDistributor(decoded));
            }
            if let Ok(decoded) =
                <SetHoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetHoldingPosition(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidityLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetLiquidityLimit(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetOwner(decoded));
            }
            if let Ok(decoded) =
                <SetPerformanceFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetPerformanceFee(decoded));
            }
            if let Ok(decoded) =
                <SetPlatformFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetPlatformFee(decoded));
            }
            if let Ok(decoded) =
                <SetRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetRebalanceDeviation(decoded));
            }
            if let Ok(decoded) =
                <SetShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetShareLockPeriod(decoded));
            }
            if let Ok(decoded) =
                <SetStrategistPayoutAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV1Calls::SetStrategistPayoutAddress(decoded));
            }
            if let Ok(decoded) =
                <SetStrategistPerformanceCutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV1Calls::SetStrategistPerformanceCut(decoded));
            }
            if let Ok(decoded) =
                <SetStrategistPlatformCutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV1Calls::SetStrategistPlatformCut(decoded));
            }
            if let Ok(decoded) =
                <SetWithdrawTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SetWithdrawType(decoded));
            }
            if let Ok(decoded) =
                <ShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::ShareLockPeriod(decoded));
            }
            if let Ok(decoded) =
                <SwapPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::SwapPositions(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::TotalAssets(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsWithdrawableCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::TotalAssetsWithdrawable(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TrustPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::TrustPosition(decoded));
            }
            if let Ok(decoded) =
                <UserShareLockStartBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::UserShareLockStartBlock(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV1Calls::WithdrawType(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CellarV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                CellarV1Calls::DomainSeparator(element) => element.encode(),
                CellarV1Calls::MaximumShareLockPeriod(element) => element.encode(),
                CellarV1Calls::MaxFeeCut(element) => element.encode(),
                CellarV1Calls::MaxPerformanceFee(element) => element.encode(),
                CellarV1Calls::MaxPlatformFee(element) => element.encode(),
                CellarV1Calls::MaxPositions(element) => element.encode(),
                CellarV1Calls::MaxRebalanceDeviation(element) => element.encode(),
                CellarV1Calls::MinimumShareLockPeriod(element) => element.encode(),
                CellarV1Calls::PriceRouterRegistrySlot(element) => element.encode(),
                CellarV1Calls::AddPosition(element) => element.encode(),
                CellarV1Calls::Allowance(element) => element.encode(),
                CellarV1Calls::AllowedRebalanceDeviation(element) => element.encode(),
                CellarV1Calls::Approve(element) => element.encode(),
                CellarV1Calls::Asset(element) => element.encode(),
                CellarV1Calls::BalanceOf(element) => element.encode(),
                CellarV1Calls::ConvertToAssets(element) => element.encode(),
                CellarV1Calls::ConvertToShares(element) => element.encode(),
                CellarV1Calls::Decimals(element) => element.encode(),
                CellarV1Calls::DecreaseAllowance(element) => element.encode(),
                CellarV1Calls::Deposit(element) => element.encode(),
                CellarV1Calls::DepositLimit(element) => element.encode(),
                CellarV1Calls::FeeData(element) => element.encode(),
                CellarV1Calls::GetPositionType(element) => element.encode(),
                CellarV1Calls::GetPositions(element) => element.encode(),
                CellarV1Calls::HoldingPosition(element) => element.encode(),
                CellarV1Calls::IncreaseAllowance(element) => element.encode(),
                CellarV1Calls::InitiateShutdown(element) => element.encode(),
                CellarV1Calls::IsPositionUsed(element) => element.encode(),
                CellarV1Calls::IsShutdown(element) => element.encode(),
                CellarV1Calls::IsTrusted(element) => element.encode(),
                CellarV1Calls::LastAccrual(element) => element.encode(),
                CellarV1Calls::LiftShutdown(element) => element.encode(),
                CellarV1Calls::LiquidityLimit(element) => element.encode(),
                CellarV1Calls::MaxDeposit(element) => element.encode(),
                CellarV1Calls::MaxMint(element) => element.encode(),
                CellarV1Calls::MaxRedeem(element) => element.encode(),
                CellarV1Calls::MaxWithdraw(element) => element.encode(),
                CellarV1Calls::Mint(element) => element.encode(),
                CellarV1Calls::Name(element) => element.encode(),
                CellarV1Calls::Nonces(element) => element.encode(),
                CellarV1Calls::Owner(element) => element.encode(),
                CellarV1Calls::Permit(element) => element.encode(),
                CellarV1Calls::Positions(element) => element.encode(),
                CellarV1Calls::PreviewDeposit(element) => element.encode(),
                CellarV1Calls::PreviewMint(element) => element.encode(),
                CellarV1Calls::PreviewRedeem(element) => element.encode(),
                CellarV1Calls::PreviewWithdraw(element) => element.encode(),
                CellarV1Calls::PushPosition(element) => element.encode(),
                CellarV1Calls::Rebalance(element) => element.encode(),
                CellarV1Calls::Redeem(element) => element.encode(),
                CellarV1Calls::Registry(element) => element.encode(),
                CellarV1Calls::RemovePosition(element) => element.encode(),
                CellarV1Calls::ResetHighWatermark(element) => element.encode(),
                CellarV1Calls::SendFees(element) => element.encode(),
                CellarV1Calls::SetDepositLimit(element) => element.encode(),
                CellarV1Calls::SetFeesDistributor(element) => element.encode(),
                CellarV1Calls::SetHoldingPosition(element) => element.encode(),
                CellarV1Calls::SetLiquidityLimit(element) => element.encode(),
                CellarV1Calls::SetOwner(element) => element.encode(),
                CellarV1Calls::SetPerformanceFee(element) => element.encode(),
                CellarV1Calls::SetPlatformFee(element) => element.encode(),
                CellarV1Calls::SetRebalanceDeviation(element) => element.encode(),
                CellarV1Calls::SetShareLockPeriod(element) => element.encode(),
                CellarV1Calls::SetStrategistPayoutAddress(element) => element.encode(),
                CellarV1Calls::SetStrategistPerformanceCut(element) => element.encode(),
                CellarV1Calls::SetStrategistPlatformCut(element) => element.encode(),
                CellarV1Calls::SetWithdrawType(element) => element.encode(),
                CellarV1Calls::ShareLockPeriod(element) => element.encode(),
                CellarV1Calls::SwapPositions(element) => element.encode(),
                CellarV1Calls::Symbol(element) => element.encode(),
                CellarV1Calls::TotalAssets(element) => element.encode(),
                CellarV1Calls::TotalAssetsWithdrawable(element) => element.encode(),
                CellarV1Calls::TotalSupply(element) => element.encode(),
                CellarV1Calls::Transfer(element) => element.encode(),
                CellarV1Calls::TransferFrom(element) => element.encode(),
                CellarV1Calls::TrustPosition(element) => element.encode(),
                CellarV1Calls::UserShareLockStartBlock(element) => element.encode(),
                CellarV1Calls::Withdraw(element) => element.encode(),
                CellarV1Calls::WithdrawType(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CellarV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarV1Calls::DomainSeparator(element) => element.fmt(f),
                CellarV1Calls::MaximumShareLockPeriod(element) => element.fmt(f),
                CellarV1Calls::MaxFeeCut(element) => element.fmt(f),
                CellarV1Calls::MaxPerformanceFee(element) => element.fmt(f),
                CellarV1Calls::MaxPlatformFee(element) => element.fmt(f),
                CellarV1Calls::MaxPositions(element) => element.fmt(f),
                CellarV1Calls::MaxRebalanceDeviation(element) => element.fmt(f),
                CellarV1Calls::MinimumShareLockPeriod(element) => element.fmt(f),
                CellarV1Calls::PriceRouterRegistrySlot(element) => element.fmt(f),
                CellarV1Calls::AddPosition(element) => element.fmt(f),
                CellarV1Calls::Allowance(element) => element.fmt(f),
                CellarV1Calls::AllowedRebalanceDeviation(element) => element.fmt(f),
                CellarV1Calls::Approve(element) => element.fmt(f),
                CellarV1Calls::Asset(element) => element.fmt(f),
                CellarV1Calls::BalanceOf(element) => element.fmt(f),
                CellarV1Calls::ConvertToAssets(element) => element.fmt(f),
                CellarV1Calls::ConvertToShares(element) => element.fmt(f),
                CellarV1Calls::Decimals(element) => element.fmt(f),
                CellarV1Calls::DecreaseAllowance(element) => element.fmt(f),
                CellarV1Calls::Deposit(element) => element.fmt(f),
                CellarV1Calls::DepositLimit(element) => element.fmt(f),
                CellarV1Calls::FeeData(element) => element.fmt(f),
                CellarV1Calls::GetPositionType(element) => element.fmt(f),
                CellarV1Calls::GetPositions(element) => element.fmt(f),
                CellarV1Calls::HoldingPosition(element) => element.fmt(f),
                CellarV1Calls::IncreaseAllowance(element) => element.fmt(f),
                CellarV1Calls::InitiateShutdown(element) => element.fmt(f),
                CellarV1Calls::IsPositionUsed(element) => element.fmt(f),
                CellarV1Calls::IsShutdown(element) => element.fmt(f),
                CellarV1Calls::IsTrusted(element) => element.fmt(f),
                CellarV1Calls::LastAccrual(element) => element.fmt(f),
                CellarV1Calls::LiftShutdown(element) => element.fmt(f),
                CellarV1Calls::LiquidityLimit(element) => element.fmt(f),
                CellarV1Calls::MaxDeposit(element) => element.fmt(f),
                CellarV1Calls::MaxMint(element) => element.fmt(f),
                CellarV1Calls::MaxRedeem(element) => element.fmt(f),
                CellarV1Calls::MaxWithdraw(element) => element.fmt(f),
                CellarV1Calls::Mint(element) => element.fmt(f),
                CellarV1Calls::Name(element) => element.fmt(f),
                CellarV1Calls::Nonces(element) => element.fmt(f),
                CellarV1Calls::Owner(element) => element.fmt(f),
                CellarV1Calls::Permit(element) => element.fmt(f),
                CellarV1Calls::Positions(element) => element.fmt(f),
                CellarV1Calls::PreviewDeposit(element) => element.fmt(f),
                CellarV1Calls::PreviewMint(element) => element.fmt(f),
                CellarV1Calls::PreviewRedeem(element) => element.fmt(f),
                CellarV1Calls::PreviewWithdraw(element) => element.fmt(f),
                CellarV1Calls::PushPosition(element) => element.fmt(f),
                CellarV1Calls::Rebalance(element) => element.fmt(f),
                CellarV1Calls::Redeem(element) => element.fmt(f),
                CellarV1Calls::Registry(element) => element.fmt(f),
                CellarV1Calls::RemovePosition(element) => element.fmt(f),
                CellarV1Calls::ResetHighWatermark(element) => element.fmt(f),
                CellarV1Calls::SendFees(element) => element.fmt(f),
                CellarV1Calls::SetDepositLimit(element) => element.fmt(f),
                CellarV1Calls::SetFeesDistributor(element) => element.fmt(f),
                CellarV1Calls::SetHoldingPosition(element) => element.fmt(f),
                CellarV1Calls::SetLiquidityLimit(element) => element.fmt(f),
                CellarV1Calls::SetOwner(element) => element.fmt(f),
                CellarV1Calls::SetPerformanceFee(element) => element.fmt(f),
                CellarV1Calls::SetPlatformFee(element) => element.fmt(f),
                CellarV1Calls::SetRebalanceDeviation(element) => element.fmt(f),
                CellarV1Calls::SetShareLockPeriod(element) => element.fmt(f),
                CellarV1Calls::SetStrategistPayoutAddress(element) => element.fmt(f),
                CellarV1Calls::SetStrategistPerformanceCut(element) => element.fmt(f),
                CellarV1Calls::SetStrategistPlatformCut(element) => element.fmt(f),
                CellarV1Calls::SetWithdrawType(element) => element.fmt(f),
                CellarV1Calls::ShareLockPeriod(element) => element.fmt(f),
                CellarV1Calls::SwapPositions(element) => element.fmt(f),
                CellarV1Calls::Symbol(element) => element.fmt(f),
                CellarV1Calls::TotalAssets(element) => element.fmt(f),
                CellarV1Calls::TotalAssetsWithdrawable(element) => element.fmt(f),
                CellarV1Calls::TotalSupply(element) => element.fmt(f),
                CellarV1Calls::Transfer(element) => element.fmt(f),
                CellarV1Calls::TransferFrom(element) => element.fmt(f),
                CellarV1Calls::TrustPosition(element) => element.fmt(f),
                CellarV1Calls::UserShareLockStartBlock(element) => element.fmt(f),
                CellarV1Calls::Withdraw(element) => element.fmt(f),
                CellarV1Calls::WithdrawType(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for CellarV1Calls {
        fn from(var: DomainSeparatorCall) -> Self {
            CellarV1Calls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<MaximumShareLockPeriodCall> for CellarV1Calls {
        fn from(var: MaximumShareLockPeriodCall) -> Self {
            CellarV1Calls::MaximumShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<MaxFeeCutCall> for CellarV1Calls {
        fn from(var: MaxFeeCutCall) -> Self {
            CellarV1Calls::MaxFeeCut(var)
        }
    }
    impl ::std::convert::From<MaxPerformanceFeeCall> for CellarV1Calls {
        fn from(var: MaxPerformanceFeeCall) -> Self {
            CellarV1Calls::MaxPerformanceFee(var)
        }
    }
    impl ::std::convert::From<MaxPlatformFeeCall> for CellarV1Calls {
        fn from(var: MaxPlatformFeeCall) -> Self {
            CellarV1Calls::MaxPlatformFee(var)
        }
    }
    impl ::std::convert::From<MaxPositionsCall> for CellarV1Calls {
        fn from(var: MaxPositionsCall) -> Self {
            CellarV1Calls::MaxPositions(var)
        }
    }
    impl ::std::convert::From<MaxRebalanceDeviationCall> for CellarV1Calls {
        fn from(var: MaxRebalanceDeviationCall) -> Self {
            CellarV1Calls::MaxRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<MinimumShareLockPeriodCall> for CellarV1Calls {
        fn from(var: MinimumShareLockPeriodCall) -> Self {
            CellarV1Calls::MinimumShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<PriceRouterRegistrySlotCall> for CellarV1Calls {
        fn from(var: PriceRouterRegistrySlotCall) -> Self {
            CellarV1Calls::PriceRouterRegistrySlot(var)
        }
    }
    impl ::std::convert::From<AddPositionCall> for CellarV1Calls {
        fn from(var: AddPositionCall) -> Self {
            CellarV1Calls::AddPosition(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CellarV1Calls {
        fn from(var: AllowanceCall) -> Self {
            CellarV1Calls::Allowance(var)
        }
    }
    impl ::std::convert::From<AllowedRebalanceDeviationCall> for CellarV1Calls {
        fn from(var: AllowedRebalanceDeviationCall) -> Self {
            CellarV1Calls::AllowedRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CellarV1Calls {
        fn from(var: ApproveCall) -> Self {
            CellarV1Calls::Approve(var)
        }
    }
    impl ::std::convert::From<AssetCall> for CellarV1Calls {
        fn from(var: AssetCall) -> Self {
            CellarV1Calls::Asset(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CellarV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            CellarV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<ConvertToAssetsCall> for CellarV1Calls {
        fn from(var: ConvertToAssetsCall) -> Self {
            CellarV1Calls::ConvertToAssets(var)
        }
    }
    impl ::std::convert::From<ConvertToSharesCall> for CellarV1Calls {
        fn from(var: ConvertToSharesCall) -> Self {
            CellarV1Calls::ConvertToShares(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CellarV1Calls {
        fn from(var: DecimalsCall) -> Self {
            CellarV1Calls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for CellarV1Calls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            CellarV1Calls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<DepositCall> for CellarV1Calls {
        fn from(var: DepositCall) -> Self {
            CellarV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<DepositLimitCall> for CellarV1Calls {
        fn from(var: DepositLimitCall) -> Self {
            CellarV1Calls::DepositLimit(var)
        }
    }
    impl ::std::convert::From<FeeDataCall> for CellarV1Calls {
        fn from(var: FeeDataCall) -> Self {
            CellarV1Calls::FeeData(var)
        }
    }
    impl ::std::convert::From<GetPositionTypeCall> for CellarV1Calls {
        fn from(var: GetPositionTypeCall) -> Self {
            CellarV1Calls::GetPositionType(var)
        }
    }
    impl ::std::convert::From<GetPositionsCall> for CellarV1Calls {
        fn from(var: GetPositionsCall) -> Self {
            CellarV1Calls::GetPositions(var)
        }
    }
    impl ::std::convert::From<HoldingPositionCall> for CellarV1Calls {
        fn from(var: HoldingPositionCall) -> Self {
            CellarV1Calls::HoldingPosition(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for CellarV1Calls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            CellarV1Calls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<InitiateShutdownCall> for CellarV1Calls {
        fn from(var: InitiateShutdownCall) -> Self {
            CellarV1Calls::InitiateShutdown(var)
        }
    }
    impl ::std::convert::From<IsPositionUsedCall> for CellarV1Calls {
        fn from(var: IsPositionUsedCall) -> Self {
            CellarV1Calls::IsPositionUsed(var)
        }
    }
    impl ::std::convert::From<IsShutdownCall> for CellarV1Calls {
        fn from(var: IsShutdownCall) -> Self {
            CellarV1Calls::IsShutdown(var)
        }
    }
    impl ::std::convert::From<IsTrustedCall> for CellarV1Calls {
        fn from(var: IsTrustedCall) -> Self {
            CellarV1Calls::IsTrusted(var)
        }
    }
    impl ::std::convert::From<LastAccrualCall> for CellarV1Calls {
        fn from(var: LastAccrualCall) -> Self {
            CellarV1Calls::LastAccrual(var)
        }
    }
    impl ::std::convert::From<LiftShutdownCall> for CellarV1Calls {
        fn from(var: LiftShutdownCall) -> Self {
            CellarV1Calls::LiftShutdown(var)
        }
    }
    impl ::std::convert::From<LiquidityLimitCall> for CellarV1Calls {
        fn from(var: LiquidityLimitCall) -> Self {
            CellarV1Calls::LiquidityLimit(var)
        }
    }
    impl ::std::convert::From<MaxDepositCall> for CellarV1Calls {
        fn from(var: MaxDepositCall) -> Self {
            CellarV1Calls::MaxDeposit(var)
        }
    }
    impl ::std::convert::From<MaxMintCall> for CellarV1Calls {
        fn from(var: MaxMintCall) -> Self {
            CellarV1Calls::MaxMint(var)
        }
    }
    impl ::std::convert::From<MaxRedeemCall> for CellarV1Calls {
        fn from(var: MaxRedeemCall) -> Self {
            CellarV1Calls::MaxRedeem(var)
        }
    }
    impl ::std::convert::From<MaxWithdrawCall> for CellarV1Calls {
        fn from(var: MaxWithdrawCall) -> Self {
            CellarV1Calls::MaxWithdraw(var)
        }
    }
    impl ::std::convert::From<MintCall> for CellarV1Calls {
        fn from(var: MintCall) -> Self {
            CellarV1Calls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for CellarV1Calls {
        fn from(var: NameCall) -> Self {
            CellarV1Calls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for CellarV1Calls {
        fn from(var: NoncesCall) -> Self {
            CellarV1Calls::Nonces(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for CellarV1Calls {
        fn from(var: OwnerCall) -> Self {
            CellarV1Calls::Owner(var)
        }
    }
    impl ::std::convert::From<PermitCall> for CellarV1Calls {
        fn from(var: PermitCall) -> Self {
            CellarV1Calls::Permit(var)
        }
    }
    impl ::std::convert::From<PositionsCall> for CellarV1Calls {
        fn from(var: PositionsCall) -> Self {
            CellarV1Calls::Positions(var)
        }
    }
    impl ::std::convert::From<PreviewDepositCall> for CellarV1Calls {
        fn from(var: PreviewDepositCall) -> Self {
            CellarV1Calls::PreviewDeposit(var)
        }
    }
    impl ::std::convert::From<PreviewMintCall> for CellarV1Calls {
        fn from(var: PreviewMintCall) -> Self {
            CellarV1Calls::PreviewMint(var)
        }
    }
    impl ::std::convert::From<PreviewRedeemCall> for CellarV1Calls {
        fn from(var: PreviewRedeemCall) -> Self {
            CellarV1Calls::PreviewRedeem(var)
        }
    }
    impl ::std::convert::From<PreviewWithdrawCall> for CellarV1Calls {
        fn from(var: PreviewWithdrawCall) -> Self {
            CellarV1Calls::PreviewWithdraw(var)
        }
    }
    impl ::std::convert::From<PushPositionCall> for CellarV1Calls {
        fn from(var: PushPositionCall) -> Self {
            CellarV1Calls::PushPosition(var)
        }
    }
    impl ::std::convert::From<RebalanceCall> for CellarV1Calls {
        fn from(var: RebalanceCall) -> Self {
            CellarV1Calls::Rebalance(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CellarV1Calls {
        fn from(var: RedeemCall) -> Self {
            CellarV1Calls::Redeem(var)
        }
    }
    impl ::std::convert::From<RegistryCall> for CellarV1Calls {
        fn from(var: RegistryCall) -> Self {
            CellarV1Calls::Registry(var)
        }
    }
    impl ::std::convert::From<RemovePositionCall> for CellarV1Calls {
        fn from(var: RemovePositionCall) -> Self {
            CellarV1Calls::RemovePosition(var)
        }
    }
    impl ::std::convert::From<ResetHighWatermarkCall> for CellarV1Calls {
        fn from(var: ResetHighWatermarkCall) -> Self {
            CellarV1Calls::ResetHighWatermark(var)
        }
    }
    impl ::std::convert::From<SendFeesCall> for CellarV1Calls {
        fn from(var: SendFeesCall) -> Self {
            CellarV1Calls::SendFees(var)
        }
    }
    impl ::std::convert::From<SetDepositLimitCall> for CellarV1Calls {
        fn from(var: SetDepositLimitCall) -> Self {
            CellarV1Calls::SetDepositLimit(var)
        }
    }
    impl ::std::convert::From<SetFeesDistributorCall> for CellarV1Calls {
        fn from(var: SetFeesDistributorCall) -> Self {
            CellarV1Calls::SetFeesDistributor(var)
        }
    }
    impl ::std::convert::From<SetHoldingPositionCall> for CellarV1Calls {
        fn from(var: SetHoldingPositionCall) -> Self {
            CellarV1Calls::SetHoldingPosition(var)
        }
    }
    impl ::std::convert::From<SetLiquidityLimitCall> for CellarV1Calls {
        fn from(var: SetLiquidityLimitCall) -> Self {
            CellarV1Calls::SetLiquidityLimit(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for CellarV1Calls {
        fn from(var: SetOwnerCall) -> Self {
            CellarV1Calls::SetOwner(var)
        }
    }
    impl ::std::convert::From<SetPerformanceFeeCall> for CellarV1Calls {
        fn from(var: SetPerformanceFeeCall) -> Self {
            CellarV1Calls::SetPerformanceFee(var)
        }
    }
    impl ::std::convert::From<SetPlatformFeeCall> for CellarV1Calls {
        fn from(var: SetPlatformFeeCall) -> Self {
            CellarV1Calls::SetPlatformFee(var)
        }
    }
    impl ::std::convert::From<SetRebalanceDeviationCall> for CellarV1Calls {
        fn from(var: SetRebalanceDeviationCall) -> Self {
            CellarV1Calls::SetRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<SetShareLockPeriodCall> for CellarV1Calls {
        fn from(var: SetShareLockPeriodCall) -> Self {
            CellarV1Calls::SetShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<SetStrategistPayoutAddressCall> for CellarV1Calls {
        fn from(var: SetStrategistPayoutAddressCall) -> Self {
            CellarV1Calls::SetStrategistPayoutAddress(var)
        }
    }
    impl ::std::convert::From<SetStrategistPerformanceCutCall> for CellarV1Calls {
        fn from(var: SetStrategistPerformanceCutCall) -> Self {
            CellarV1Calls::SetStrategistPerformanceCut(var)
        }
    }
    impl ::std::convert::From<SetStrategistPlatformCutCall> for CellarV1Calls {
        fn from(var: SetStrategistPlatformCutCall) -> Self {
            CellarV1Calls::SetStrategistPlatformCut(var)
        }
    }
    impl ::std::convert::From<SetWithdrawTypeCall> for CellarV1Calls {
        fn from(var: SetWithdrawTypeCall) -> Self {
            CellarV1Calls::SetWithdrawType(var)
        }
    }
    impl ::std::convert::From<ShareLockPeriodCall> for CellarV1Calls {
        fn from(var: ShareLockPeriodCall) -> Self {
            CellarV1Calls::ShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<SwapPositionsCall> for CellarV1Calls {
        fn from(var: SwapPositionsCall) -> Self {
            CellarV1Calls::SwapPositions(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CellarV1Calls {
        fn from(var: SymbolCall) -> Self {
            CellarV1Calls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalAssetsCall> for CellarV1Calls {
        fn from(var: TotalAssetsCall) -> Self {
            CellarV1Calls::TotalAssets(var)
        }
    }
    impl ::std::convert::From<TotalAssetsWithdrawableCall> for CellarV1Calls {
        fn from(var: TotalAssetsWithdrawableCall) -> Self {
            CellarV1Calls::TotalAssetsWithdrawable(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CellarV1Calls {
        fn from(var: TotalSupplyCall) -> Self {
            CellarV1Calls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CellarV1Calls {
        fn from(var: TransferCall) -> Self {
            CellarV1Calls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CellarV1Calls {
        fn from(var: TransferFromCall) -> Self {
            CellarV1Calls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TrustPositionCall> for CellarV1Calls {
        fn from(var: TrustPositionCall) -> Self {
            CellarV1Calls::TrustPosition(var)
        }
    }
    impl ::std::convert::From<UserShareLockStartBlockCall> for CellarV1Calls {
        fn from(var: UserShareLockStartBlockCall) -> Self {
            CellarV1Calls::UserShareLockStartBlock(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for CellarV1Calls {
        fn from(var: WithdrawCall) -> Self {
            CellarV1Calls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawTypeCall> for CellarV1Calls {
        fn from(var: WithdrawTypeCall) -> Self {
            CellarV1Calls::WithdrawType(var)
        }
    }
}