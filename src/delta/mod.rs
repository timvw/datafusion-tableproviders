use std::sync::Arc;
use datafusion::datasource::datasource::TableProviderFactory;
use datafusion::datasource::TableProvider;

pub struct DeltaTableProviderFactory {
}

impl TableProviderFactory for DeltaTableProviderFactory {
    fn create(&self, name: &str, url: &str) -> Arc<dyn TableProvider> {
        todo!()
    }
}