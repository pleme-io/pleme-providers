//! # pleme-providers
//!
//! Multi-provider integration library for Pleme platform services.
//!
//! ## Philosophy
//!
//! This library implements the **Strategy Pattern** for integrating multiple external providers:
//! - Single interface for multiple implementations
//! - Runtime provider selection via registry
//! - Consistent error handling across providers
//! - Capability negotiation
//!
//! ## Use Cases
//!
//! - **Product Catalog**: Multiple dropshipping providers (CJ, EPROLO, Alibaba, etc.)
//! - **Payment Processing**: Multiple payment gateways (Stripe, PayPal, Mercado Pago, PIX)
//! - **Shipping**: Multiple carriers (Correios, FedEx, DHL)
//! - **Email**: Multiple email services (SES, SendGrid, Mailgun)
//! - **SMS**: Multiple SMS providers (Twilio, Vonage)
//!
//! ## Usage
//!
//! ```rust
//! use pleme_providers::{Provider, ProviderRegistry, ProviderCapabilities, ProviderBatch, ProviderError};
//! use async_trait::async_trait;
//!
//! // Define your domain-specific item type
//! #[derive(Clone)]
//! struct Product {
//!     id: String,
//!     name: String,
//! }
//!
//! // Implement the Provider trait for your specific provider
//! struct MyDropshipProvider;
//!
//! #[async_trait]
//! impl Provider for MyDropshipProvider {
//!     type Item = Product;
//!     type Filter = ();
//!     type Pagination = ();
//!
//!     fn provider_id(&self) -> &str { "my-provider" }
//!     fn provider_name(&self) -> &str { "My Provider" }
//!     fn capabilities(&self) -> ProviderCapabilities {
//!         ProviderCapabilities::default()
//!     }
//!
//!     async fn fetch_items(
//!         &self,
//!         _filter: Self::Filter,
//!         _pagination: Self::Pagination,
//!     ) -> Result<ProviderBatch<Self::Item>, ProviderError> {
//!         Ok(ProviderBatch::empty())
//!     }
//!
//!     async fn validate_credentials(&self) -> Result<(), ProviderError> {
//!         Ok(())
//!     }
//! }
//!
//! // Use the registry to manage multiple providers
//! # async fn example() {
//! let mut registry = ProviderRegistry::new();
//! registry.register(std::sync::Arc::new(MyDropshipProvider));
//!
//! let provider = registry.get("my-provider").expect("Provider not found");
//! # }
//! ```

pub mod provider;
pub mod registry;
pub mod capabilities;
pub mod error;
pub mod batch;

pub use provider::Provider;
pub use registry::ProviderRegistry;
pub use capabilities::ProviderCapabilities;
pub use error::ProviderError;
pub use batch::ProviderBatch;

/// Result type for provider operations
pub type Result<T> = std::result::Result<T, ProviderError>;
