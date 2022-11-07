use std::sync::Arc;
use datafusion::datasource::datasource::TableProviderFactory;
use datafusion::datasource::TableProvider;

pub struct PostgresTableProviderFactory {
}

impl TableProviderFactory for PostgresTableProviderFactory {
    fn create(&self, name: &str, url: &str) -> Arc<dyn TableProvider> {
        todo!()
    }
}