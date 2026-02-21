//! Generic provider trait for multi-provider integrations

use async_trait::async_trait;
use crate::{ProviderCapabilities, ProviderError, ProviderBatch};

/// Generic provider trait for integrating multiple external providers
///
/// This trait defines the interface that all provider implementations must follow,
/// enabling services to integrate with multiple external sources through a consistent API.
///
/// ## Type Parameters
///
/// - `Item`: The type of items returned by this provider (e.g., Product, Payment, Shipment)
/// - `Filter`: Domain-specific filter criteria for fetching items
/// - `Pagination`: Pagination parameters for the provider's API
///
/// ## Example
///
/// ```rust
/// use pleme_providers::{Provider, ProviderCapabilities, ProviderError, ProviderBatch};
/// use async_trait::async_trait;
///
/// #[derive(Clone)]
/// struct Product { id: String, name: String }
///
/// #[derive(Default)]
/// struct ProductFilter { category: Option<String> }
///
/// #[derive(Default)]
/// struct ProductPagination { page: i32, per_page: i32 }
///
/// struct MyProvider;
///
/// #[async_trait]
/// impl Provider for MyProvider {
///     type Item = Product;
///     type Filter = ProductFilter;
///     type Pagination = ProductPagination;
///
///     fn provider_id(&self) -> &str { "my-provider" }
///     fn provider_name(&self) -> &str { "My Provider" }
///     fn capabilities(&self) -> ProviderCapabilities { ProviderCapabilities::default() }
///
///     async fn fetch_items(
///         &self,
///         _filter: Self::Filter,
///         _pagination: Self::Pagination,
///     ) -> Result<ProviderBatch<Self::Item>, ProviderError> {
///         // Implement provider-specific logic
///         Ok(ProviderBatch {
///             items: vec![],
///             total_count: 0,
///             has_next_page: false,
///             next_page_token: None,
///         })
///     }
///
///     async fn validate_credentials(&self) -> Result<(), ProviderError> {
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Provider: Send + Sync {
    /// The type of items this provider returns (e.g., Product, Payment, Shipment)
    type Item: Send + Sync;

    /// Domain-specific filter type for fetching items
    type Filter: Send + Sync;

    /// Pagination parameters type
    type Pagination: Send + Sync;

    /// Unique identifier for the provider (e.g., "stripe", "cj_dropshipping", "fedex")
    ///
    /// This should be a lowercase, kebab-case string that uniquely identifies the provider.
    fn provider_id(&self) -> &str;

    /// Human-readable provider name (e.g., "Stripe", "CJ Dropshipping", "FedEx")
    fn provider_name(&self) -> &str;

    /// Provider capabilities (webhooks, real-time updates, etc.)
    fn capabilities(&self) -> ProviderCapabilities;

    /// Fetch items from provider with filtering and pagination
    ///
    /// # Arguments
    ///
    /// * `filter` - Domain-specific filter criteria
    /// * `pagination` - Pagination parameters
    ///
    /// # Returns
    ///
    /// * `Ok(ProviderBatch)` - Batch of items with pagination metadata
    /// * `Err(ProviderError)` - Network error, API error, or rate limit exceeded
    async fn fetch_items(
        &self,
        filter: Self::Filter,
        pagination: Self::Pagination,
    ) -> Result<ProviderBatch<Self::Item>, ProviderError>;

    /// Validate provider authentication credentials
    ///
    /// This method should verify that the provider's API credentials are valid
    /// and that the service can successfully communicate with the provider.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Credentials are valid
    /// * `Err(ProviderError)` - Invalid credentials or authentication failed
    async fn validate_credentials(&self) -> Result<(), ProviderError>;

    /// Optional: Get provider-specific metadata
    ///
    /// Override this method to provide additional provider-specific information
    /// (e.g., supported regions, available features, pricing tiers)
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}
