//! Provider error types

use thiserror::Error;

/// Errors that can occur during provider operations
///
/// These errors cover common failure modes across all provider types,
/// including network errors, API errors, authentication failures, and rate limiting.
#[derive(Error, Debug)]
pub enum ProviderError {
    /// Provider API returned an error response
    #[error("Provider API error (status {status}): {message}")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message from provider
        message: String,
    },

    /// Rate limit exceeded for this provider
    #[error("Rate limit exceeded for provider {provider_id}")]
    RateLimitExceeded {
        /// Provider identifier
        provider_id: String,
    },

    /// Network error occurred during communication with provider
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Invalid or malformed data received from provider
    #[error("Invalid data from provider: {0}")]
    InvalidData(String),

    /// Requested resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Authentication or authorization failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Provider is temporarily unavailable
    #[error("Provider unavailable: {0}")]
    ProviderUnavailable(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Operation timeout
    #[error("Operation timed out after {timeout_ms}ms")]
    Timeout {
        /// Timeout duration in milliseconds
        timeout_ms: u64,
    },

    /// General error with custom message
    #[error("Provider error: {0}")]
    Other(String),
}

impl ProviderError {
    /// Create an API error
    pub fn api_error(status: u16, message: impl Into<String>) -> Self {
        Self::ApiError {
            status,
            message: message.into(),
        }
    }

    /// Create a rate limit error
    pub fn rate_limit(provider_id: impl Into<String>) -> Self {
        Self::RateLimitExceeded {
            provider_id: provider_id.into(),
        }
    }

    /// Create a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::NetworkError(message.into())
    }

    /// Create an invalid data error
    pub fn invalid_data(message: impl Into<String>) -> Self {
        Self::InvalidData(message.into())
    }

    /// Create a not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound(resource.into())
    }

    /// Create an authentication error
    pub fn auth_failed(message: impl Into<String>) -> Self {
        Self::AuthenticationFailed(message.into())
    }

    /// Create a provider unavailable error
    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::ProviderUnavailable(message.into())
    }

    /// Create a timeout error
    pub fn timeout(timeout_ms: u64) -> Self {
        Self::Timeout { timeout_ms }
    }

    /// Check if this is a retriable error
    ///
    /// Returns `true` for errors that may succeed on retry (network errors, timeouts,
    /// provider unavailable), and `false` for permanent errors (authentication, not found).
    pub fn is_retriable(&self) -> bool {
        matches!(
            self,
            Self::NetworkError(_)
                | Self::ProviderUnavailable(_)
                | Self::Timeout { .. }
                | Self::RateLimitExceeded { .. }
        )
    }

    /// Check if this is an authentication error
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::AuthenticationFailed(_))
    }

    /// Check if this is a rate limit error
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Self::RateLimitExceeded { .. })
    }
}

// Optional reqwest integration
#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for ProviderError {
    fn from(err: reqwest::Error) -> Self {
        Self::network(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error() {
        let err = ProviderError::api_error(404, "Not found");
        assert!(matches!(err, ProviderError::ApiError { status: 404, .. }));
        assert_eq!(err.to_string(), "Provider API error (status 404): Not found");
    }

    #[test]
    fn test_rate_limit() {
        let err = ProviderError::rate_limit("test-provider");
        assert!(matches!(err, ProviderError::RateLimitExceeded { .. }));
        assert!(err.is_rate_limit());
        assert!(err.is_retriable());
    }

    #[test]
    fn test_network_error() {
        let err = ProviderError::network("Connection refused");
        assert!(matches!(err, ProviderError::NetworkError(_)));
        assert!(err.is_retriable());
    }

    #[test]
    fn test_auth_error() {
        let err = ProviderError::auth_failed("Invalid API key");
        assert!(err.is_auth_error());
        assert!(!err.is_retriable());
    }

    #[test]
    fn test_timeout_error() {
        let err = ProviderError::timeout(5000);
        assert!(matches!(err, ProviderError::Timeout { timeout_ms: 5000 }));
        assert!(err.is_retriable());
    }

    #[test]
    fn test_not_found_not_retriable() {
        let err = ProviderError::not_found("Product 123");
        assert!(!err.is_retriable());
    }
}
