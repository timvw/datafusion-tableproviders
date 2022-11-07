#![allow(unused_imports)]
#![allow(unused_variables)]
use std::sync::Arc;
use datafusion::prelude::*;
use datafusion::common::Result;

#[cfg(feature = "delta")]
pub mod delta;
#[cfg(feature = "postgres")]
pub mod postgres;

pub fn register_all_enabled_factories(ctx: &mut SessionContext) -> Result<()> {

    #[cfg(feature = "delta")]
    let _ = ctx.table_factories.insert(String::from("DELTA"), Arc::new(delta::DeltaTableProviderFactory{}));

    #[cfg(feature = "postgres")]
    let _ = ctx.table_factories.insert(String::from("POSTGRES"), Arc::new(postgres::PostgresTableProviderFactory{}));

    Ok(())
}