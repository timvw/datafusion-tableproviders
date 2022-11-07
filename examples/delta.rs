use datafusion::prelude::*;
use datafusion::common::Result;

use datafusion_tableproviders::*;

#[tokio::main]
async fn main() -> Result<()> {

    let mut ctx = SessionContext::new();

    let _ = register_all_enabled_factories(&mut ctx)?;

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
