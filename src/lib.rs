//! aniSearch (<https://www.anisearch.com>) anime metadata provider extension
//! for LisekMovie — **skeleton**.
//!
//! aniSearch is the largest German-language anime database (German,
//! English, Italian, Spanish, French and Japanese synopses). It has no
//! public API: its data is only offered to partners on request. This crate
//! already takes its place in the provider registry — Admin → Plugins
//! lists it, libraries can order it, Fix match accepts its ids
//! (`anisearch:2227`, `https://www.anisearch.com/anime/2227,one-piece`) —
//! and every method answers "nothing found" until the endpoints are
//! implemented.
//!
//! TODO: implement endpoint — once a partner API key (or a permitted way of
//! reading the anime pages) exists: `search` (title → candidates),
//! `lookup` (id → titles, synopsis, genres, rating, cover) and the
//! automatic `find_movie` / `find_series`.

use std::sync::Once;

use async_trait::async_trait;
use lisek_plugin_api::{
    MediaKind, MetadataProvider, MovieQuery, ProviderCandidate, ProviderError, ProviderMatch,
    SeriesQuery,
};

static TODO: Once = Once::new();

/// Logs the missing implementation once per process, not once per scanned
/// file.
fn warn_todo(what: &str) {
    TODO.call_once(|| {
        tracing::warn!(
            provider = "anisearch",
            first_call = what,
            "TODO: implement endpoint — aniSearch has no public API yet; this provider returns no results"
        );
    });
}

/// The [`MetadataProvider`] for `"anisearch"`.
pub struct AnisearchProvider {
    #[allow(dead_code)] // the partner key, once the endpoints exist
    api_key: String,
}

impl AnisearchProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }
}

#[async_trait]
impl MetadataProvider for AnisearchProvider {
    fn provider_key(&self) -> &'static str {
        "anisearch"
    }

    async fn find_movie(
        &self,
        _query: &MovieQuery<'_>,
        _lang: &str,
    ) -> Result<Option<ProviderMatch>, ProviderError> {
        warn_todo("find_movie");
        Ok(None)
    }

    async fn find_series(
        &self,
        _query: &SeriesQuery<'_>,
        _lang: &str,
    ) -> Result<Option<ProviderMatch>, ProviderError> {
        warn_todo("find_series");
        Ok(None)
    }

    async fn search(
        &self,
        _kind: MediaKind,
        _title: &str,
        _year: Option<i32>,
        _lang: &str,
    ) -> Result<Vec<ProviderCandidate>, ProviderError> {
        warn_todo("search");
        Ok(Vec::new())
    }

    async fn lookup(
        &self,
        _kind: MediaKind,
        _provider_id: &str,
        _lang: &str,
    ) -> Result<Option<ProviderMatch>, ProviderError> {
        warn_todo("lookup");
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lisek_plugin_api::DirectId;

    #[tokio::test]
    async fn answers_nothing_until_implemented() {
        let provider = AnisearchProvider::new("");
        assert_eq!(provider.provider_key(), "anisearch");
        assert!(
            provider
                .search(MediaKind::Series, "One Piece", None, "de")
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            provider
                .find_series(&SeriesQuery { title: "One Piece" }, "de")
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            provider
                .lookup_by_id(MediaKind::Series, &DirectId::Numeric("2227".into()), "de")
                .await
                .unwrap()
                .is_none()
        );
    }
}
