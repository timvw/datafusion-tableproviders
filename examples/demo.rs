use std::sync::Arc;
use datafusion::prelude::*;
use datafusion::common::Result;
use datafusion::datasource::datasource::TableProviderFactory;
use datafusion::datasource::TableProvider;

use datafusion_tableproviders::*;

#[tokio::main]
async fn main() -> Result<()> {

    let mut ctx = SessionContext::new();

    ctx = register_all_enabled_factories(ctx)?;

    let _ = ctx
        .sql("CREATE EXTERNAL TABLE x STORED AS DELTA LOCATION './testing'")
        .await?;

    let df = ctx
        .sql("SELECT * FROM x")
        .await?;

    df
        .show_limit(10)
        .await?;

    Ok(())
}
