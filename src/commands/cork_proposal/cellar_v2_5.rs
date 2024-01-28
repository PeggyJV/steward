use abscissa_core::{clap::Parser, Command, Runnable};

mod add_adaptor_to_catalogue;
mod add_position_to_catalogue;
mod cache_price_router;
mod force_position_out;
mod increase_share_supply_cap;
mod set_automation_actions;
mod set_rebalance_deviation;
mod set_share_price_oracle;
mod set_strategist_platform_cut;
mod toggle_ignore_pause;

/// Generates scheduled cork proposal templates containing function call data for V2.5 cellars
#[derive(Command, Debug, Parser, Runnable)]
pub enum CellarV2_5Cmd {
    AddAdaptorToCatalogue(add_adaptor_to_catalogue::AddAdaptorToCatalogueCmd),
    AddPositionToCatalogue(add_position_to_catalogue::AddPositionToCatalogueCmd),
    ForcePositionOut(force_position_out::ForcePositionOutCmd),
    SetRebalanceDeviation(set_rebalance_deviation::SetRebalanceDeviationCmd),
    SetStrategistPlatformCut(set_strategist_platform_cut::SetStrategistPlatformCutCmd),
    ToggleIgnorePause(toggle_ignore_pause::ToggleIgnorePauseCmd),
    SetSharePriceOracle(set_share_price_oracle::SetSharePriceOracleCmd),
    IncreaseShareSupplyCap(increase_share_supply_cap::IncreaseShareSupplyCapCmd),
    SetAutomationActions(set_automation_actions::SetAutomationActionsCmd),
    CachePriceRouter(cache_price_router::CachePriceRouterCmd),
}
