//! Provider capabilities and feature flags

use serde::{Deserialize, Serialize};

/// Provider capabilities and limitations
///
/// Describes what features and operations a provider supports, allowing
/// services to adapt their behavior based on provider capabilities.
///
/// ## Example
///
/// ```rust
/// use pleme_providers::ProviderCapabilities;
///
/// let capabilities = ProviderCapabilities {
///     supports_webhooks: true,
///     supports_real_time_updates: true,
///     rate_limit_per_second: 100,
///     pagination_max_size: 1000,
///     requires_authentication: true,
///     supports_batch_operations: false,
///     ..Default::default()
/// };
///
/// if capabilities.supports_webhooks {
///     println!("Provider supports webhook notifications");
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCapabilities {
    /// Provider supports webhook notifications for real-time updates
    pub supports_webhooks: bool,

    /// Provider supports real-time queries (vs batch sync only)
    pub supports_real_time_updates: bool,

    /// Provider supports batch operations (e.g., bulk updates)
    pub supports_batch_operations: bool,

    /// Maximum requests per second to respect rate limiting
    ///
    /// Set to 0 for no rate limiting. Most providers have rate limits
    /// between 1-100 requests per second.
    pub rate_limit_per_second: u32,

    /// Maximum number of items per page in API responses
    ///
    /// Common values: 50, 100, 1000. Used for pagination.
    pub pagination_max_size: u32,

    /// Provider requires authentication (API key, OAuth, etc.)
    pub requires_authentication: bool,

    /// Provider supports cursor-based pagination (vs offset-based)
    pub supports_cursor_pagination: bool,

    /// Provider supports filtering/search
    pub supports_filtering: bool,

    /// Provider supports sorting results
    pub supports_sorting: bool,
}

impl Default for ProviderCapabilities {
    fn default() -> Self {
        Self {
            supports_webhooks: false,
            supports_real_time_updates: false,
            supports_batch_operations: false,
            rate_limit_per_second: 10,
            pagination_max_size: 50,
            requires_authentication: true,
            supports_cursor_pagination: false,
            supports_filtering: false,
            supports_sorting: false,
        }
    }
}

impl ProviderCapabilities {
    /// Check if provider supports a minimum rate limit
    ///
    /// # Arguments
    ///
    /// * `min_rate` - Minimum requests per second required
    ///
    /// # Returns
    ///
    /// `true` if provider supports at least the minimum rate, `false` otherwise
    pub fn meets_rate_limit(&self, min_rate: u32) -> bool {
        self.rate_limit_per_second == 0 || self.rate_limit_per_second >= min_rate
    }

    /// Check if provider supports a minimum pagination size
    ///
    /// # Arguments
    ///
    /// * `min_size` - Minimum page size required
    ///
    /// # Returns
    ///
    /// `true` if provider supports at least the minimum page size, `false` otherwise
    pub fn meets_pagination_size(&self, min_size: u32) -> bool {
        self.pagination_max_size >= min_size
    }

    /// Get recommended page size for this provider
    ///
    /// Returns a safe page size that respects the provider's limits.
    /// Defaults to the provider's max size, or 50 if not specified.
    pub fn recommended_page_size(&self) -> u32 {
        if self.pagination_max_size > 0 {
            self.pagination_max_size
        } else {
            50
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capabilities() {
        let caps = ProviderCapabilities::default();
        assert!(!caps.supports_webhooks);
        assert!(!caps.supports_real_time_updates);
        assert_eq!(caps.rate_limit_per_second, 10);
        assert_eq!(caps.pagination_max_size, 50);
        assert!(caps.requires_authentication);
    }

    #[test]
    fn test_meets_rate_limit() {
        let caps = ProviderCapabilities {
            rate_limit_per_second: 100,
            ..Default::default()
        };

        assert!(caps.meets_rate_limit(50));
        assert!(caps.meets_rate_limit(100));
        assert!(!caps.meets_rate_limit(150));
    }

    #[test]
    fn test_meets_rate_limit_unlimited() {
        let caps = ProviderCapabilities {
            rate_limit_per_second: 0, // Unlimited
            ..Default::default()
        };

        assert!(caps.meets_rate_limit(1000));
        assert!(caps.meets_rate_limit(10000));
    }

    #[test]
    fn test_meets_pagination_size() {
        let caps = ProviderCapabilities {
            pagination_max_size: 100,
            ..Default::default()
        };

        assert!(caps.meets_pagination_size(50));
        assert!(caps.meets_pagination_size(100));
        assert!(!caps.meets_pagination_size(150));
    }

    #[test]
    fn test_recommended_page_size() {
        let caps = ProviderCapabilities {
            pagination_max_size: 100,
            ..Default::default()
        };

        assert_eq!(caps.recommended_page_size(), 100);

        let caps_zero = ProviderCapabilities {
            pagination_max_size: 0,
            ..Default::default()
        };

        assert_eq!(caps_zero.recommended_page_size(), 50);
    }
}
