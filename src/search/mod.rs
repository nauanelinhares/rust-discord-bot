pub mod error;
pub mod youtube;

use async_trait::async_trait;

use crate::search::error::SearchError;

#[async_trait]
pub trait Search {
    async fn search(
        &self,
        query: &str,
    ) -> Result<Vec<crate::search::youtube::YoutubeVideo>, SearchError>;
}

pub use youtube::{YoutubeSearch, YoutubeVideo};
