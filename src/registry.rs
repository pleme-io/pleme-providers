//! Provider registry for managing multiple provider implementations

use std::collections::HashMap;
use std::sync::Arc;
use crate::Provider;

/// Registry for managing multiple provider implementations
///
/// The `ProviderRegistry` allows you to register and retrieve providers at runtime,
/// enabling dynamic provider selection based on configuration or user preferences.
///
/// ## Example
///
/// ```rust
/// use pleme_providers::{Provider, ProviderRegistry};
/// use std::sync::Arc;
/// # use async_trait::async_trait;
/// # use pleme_providers::{ProviderCapabilities, ProviderError, ProviderBatch};
/// # #[derive(Clone)] struct Product;
/// # struct MyProvider { id: String, name: String }
/// # #[async_trait] impl Provider for MyProvider {
/// #     type Item = Product; type Filter = (); type Pagination = ();
/// #     fn provider_id(&self) -> &str { &self.id }
/// #     fn provider_name(&self) -> &str { &self.name }
/// #     fn capabilities(&self) -> ProviderCapabilities { ProviderCapabilities::default() }
/// #     async fn fetch_items(&self, _f: (), _p: ()) -> Result<ProviderBatch<Product>, ProviderError> {
/// #         Ok(ProviderBatch::empty())
/// #     }
/// #     async fn validate_credentials(&self) -> Result<(), ProviderError> { Ok(()) }
/// # }
///
/// let mut registry = ProviderRegistry::new();
///
/// // Register providers with different configurations
/// registry.register(Arc::new(MyProvider {
///     id: "provider1".to_string(),
///     name: "Provider 1".to_string(),
/// }));
/// registry.register(Arc::new(MyProvider {
///     id: "provider2".to_string(),
///     name: "Provider 2".to_string(),
/// }));
///
/// // Retrieve providers
/// let provider1 = registry.get("provider1").expect("Provider not found");
/// let provider2 = registry.get("provider2").expect("Provider not found");
///
/// // List all providers
/// let all_provider_ids = registry.provider_ids();
/// assert_eq!(all_provider_ids, vec!["provider1", "provider2"]);
/// ```
pub struct ProviderRegistry<P: Provider + ?Sized> {
    providers: HashMap<String, Arc<P>>,
}

impl<P: Provider + ?Sized> ProviderRegistry<P> {
    /// Create a new empty provider registry
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider
    ///
    /// If a provider with the same ID already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `provider` - Arc-wrapped provider implementation
    pub fn register(&mut self, provider: Arc<P>) {
        let provider_id = provider.provider_id().to_string();
        self.providers.insert(provider_id, provider);
    }

    /// Get a provider by its ID
    ///
    /// # Arguments
    ///
    /// * `provider_id` - Unique provider identifier
    ///
    /// # Returns
    ///
    /// * `Some(Arc<P>)` - Provider if found
    /// * `None` - Provider not found
    pub fn get(&self, provider_id: &str) -> Option<Arc<P>> {
        self.providers.get(provider_id).cloned()
    }

    /// Get all registered providers
    ///
    /// # Returns
    ///
    /// Vector of Arc-wrapped providers
    pub fn all_providers(&self) -> Vec<Arc<P>> {
        self.providers.values().cloned().collect()
    }

    /// Get all provider IDs
    ///
    /// # Returns
    ///
    /// Sorted vector of provider IDs
    pub fn provider_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.providers.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Check if a provider is registered
    ///
    /// # Arguments
    ///
    /// * `provider_id` - Unique provider identifier
    ///
    /// # Returns
    ///
    /// `true` if provider is registered, `false` otherwise
    pub fn contains(&self, provider_id: &str) -> bool {
        self.providers.contains_key(provider_id)
    }

    /// Remove a provider from the registry
    ///
    /// # Arguments
    ///
    /// * `provider_id` - Unique provider identifier
    ///
    /// # Returns
    ///
    /// * `Some(Arc<P>)` - Removed provider
    /// * `None` - Provider not found
    pub fn remove(&mut self, provider_id: &str) -> Option<Arc<P>> {
        self.providers.remove(provider_id)
    }

    /// Get the number of registered providers
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}

impl<P: Provider + ?Sized> Default for ProviderRegistry<P> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::{ProviderCapabilities, ProviderError, ProviderBatch};

    #[derive(Clone)]
    struct TestItem;

    struct TestProvider {
        id: String,
        name: String,
    }

    #[async_trait]
    impl Provider for TestProvider {
        type Item = TestItem;
        type Filter = ();
        type Pagination = ();

        fn provider_id(&self) -> &str {
            &self.id
        }

        fn provider_name(&self) -> &str {
            &self.name
        }

        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities::default()
        }

        async fn fetch_items(
            &self,
            _filter: Self::Filter,
            _pagination: Self::Pagination,
        ) -> Result<ProviderBatch<Self::Item>, ProviderError> {
            Ok(ProviderBatch {
                items: vec![],
                total_count: 0,
                has_next_page: false,
                next_page_token: None,
            })
        }

        async fn validate_credentials(&self) -> Result<(), ProviderError> {
            Ok(())
        }
    }

    #[test]
    fn test_registry_registration() {
        let mut registry = ProviderRegistry::new();
        assert!(registry.is_empty());

        let provider = Arc::new(TestProvider {
            id: "test-provider".to_string(),
            name: "Test Provider".to_string(),
        });

        registry.register(provider);
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_registry_get() {
        let mut registry = ProviderRegistry::new();

        let provider = Arc::new(TestProvider {
            id: "test-provider".to_string(),
            name: "Test Provider".to_string(),
        });

        registry.register(provider);

        let retrieved = registry.get("test-provider");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().provider_id(), "test-provider");

        let not_found = registry.get("non-existent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_registry_provider_ids() {
        let mut registry = ProviderRegistry::new();

        registry.register(Arc::new(TestProvider {
            id: "provider-a".to_string(),
            name: "Provider A".to_string(),
        }));

        registry.register(Arc::new(TestProvider {
            id: "provider-b".to_string(),
            name: "Provider B".to_string(),
        }));

        let ids = registry.provider_ids();
        assert_eq!(ids, vec!["provider-a", "provider-b"]);
    }

    #[test]
    fn test_registry_contains() {
        let mut registry = ProviderRegistry::new();

        let provider = Arc::new(TestProvider {
            id: "test-provider".to_string(),
            name: "Test Provider".to_string(),
        });

        registry.register(provider);

        assert!(registry.contains("test-provider"));
        assert!(!registry.contains("non-existent"));
    }

    #[test]
    fn test_registry_remove() {
        let mut registry = ProviderRegistry::new();

        let provider = Arc::new(TestProvider {
            id: "test-provider".to_string(),
            name: "Test Provider".to_string(),
        });

        registry.register(provider);
        assert_eq!(registry.len(), 1);

        let removed = registry.remove("test-provider");
        assert!(removed.is_some());
        assert_eq!(registry.len(), 0);

        let not_found = registry.remove("non-existent");
        assert!(not_found.is_none());
    }
}
