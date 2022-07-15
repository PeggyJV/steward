// Clippy wants enum variants that are significantly large to be in a Box<T>.
// Since this is generated code we just ignore this rule.
#[allow(clippy::large_enum_variant, clippy::extra_unused_lifetimes)]
pub mod aave_v2_stablecoin;
