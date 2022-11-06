use std::sync::Arc;
use datafusion::prelude::*;
use datafusion::common::Result;
use crate::delta::DeltaTableProviderFactory;

#[cfg(feature = "delta")]
pub mod delta;

pub fn register_all_enabled_factories(mut ctx: SessionContext) -> Result<SessionContext> {

    if cfg!(feature = "delta") {
        let delta_table_provider_factory = DeltaTableProviderFactory{};
        let _ = ctx.table_factories.insert(String::from("DELTA"), Arc::new(delta_table_provider_factory));
    }

    Ok(ctx)
}