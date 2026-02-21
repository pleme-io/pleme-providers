//! Provider batch results with pagination metadata

use serde::{Deserialize, Serialize};

/// Batch of items returned from a provider with pagination metadata
///
/// This type is used to represent paginated results from provider APIs,
/// including the items themselves and metadata about the pagination state.
///
/// ## Example
///
/// ```rust
/// use pleme_providers::ProviderBatch;
///
/// #[derive(Clone)]
/// struct Product {
///     id: String,
///     name: String,
/// }
///
/// let batch = ProviderBatch {
///     items: vec![
///         Product { id: "1".to_string(), name: "Product 1".to_string() },
///         Product { id: "2".to_string(), name: "Product 2".to_string() },
///     ],
///     total_count: 100,
///     has_next_page: true,
///     next_page_token: Some("cursor-123".to_string()),
/// };
///
/// println!("Got {} items out of {} total", batch.items.len(), batch.total_count);
///
/// if batch.has_next_page {
///     println!("More items available");
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderBatch<T> {
    /// Items in this batch
    pub items: Vec<T>,

    /// Total number of items matching the query (across all pages)
    ///
    /// Some providers may not provide an accurate total count. In such cases,
    /// this value may be an estimate or 0.
    pub total_count: i64,

    /// Whether there are more pages available
    ///
    /// This should be `true` if calling fetch_items with the next page
    /// would return additional results.
    pub has_next_page: bool,

    /// Token for cursor-based pagination
    ///
    /// Some providers use cursor-based pagination instead of offset-based.
    /// If the provider supports this, `next_page_token` will contain the
    /// cursor to use for fetching the next page.
    pub next_page_token: Option<String>,
}

impl<T> ProviderBatch<T> {
    /// Create a new batch with items
    pub fn new(items: Vec<T>) -> Self {
        let count = items.len() as i64;
        Self {
            items,
            total_count: count,
            has_next_page: false,
            next_page_token: None,
        }
    }

    /// Create a new batch with pagination metadata
    pub fn with_pagination(
        items: Vec<T>,
        total_count: i64,
        has_next_page: bool,
        next_page_token: Option<String>,
    ) -> Self {
        Self {
            items,
            total_count,
            has_next_page,
            next_page_token,
        }
    }

    /// Create an empty batch
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            total_count: 0,
            has_next_page: false,
            next_page_token: None,
        }
    }

    /// Get the number of items in this batch
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if this batch is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Check if this is the last page
    pub fn is_last_page(&self) -> bool {
        !self.has_next_page
    }

    /// Get a reference to the items
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Consume the batch and return the items
    pub fn into_items(self) -> Vec<T> {
        self.items
    }

    /// Map the items in this batch to a different type
    pub fn map<U, F>(self, f: F) -> ProviderBatch<U>
    where
        F: FnMut(T) -> U,
    {
        ProviderBatch {
            items: self.items.into_iter().map(f).collect(),
            total_count: self.total_count,
            has_next_page: self.has_next_page,
            next_page_token: self.next_page_token,
        }
    }

    /// Filter the items in this batch
    pub fn filter<F>(self, mut f: F) -> ProviderBatch<T>
    where
        F: FnMut(&T) -> bool,
    {
        let filtered_items: Vec<T> = self.items.into_iter().filter(|item| f(item)).collect();
        let new_count = filtered_items.len() as i64;

        ProviderBatch {
            items: filtered_items,
            total_count: new_count,
            has_next_page: self.has_next_page,
            next_page_token: self.next_page_token,
        }
    }
}

impl<T> Default for ProviderBatch<T> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_batch() {
        let batch = ProviderBatch::new(vec![1, 2, 3]);
        assert_eq!(batch.len(), 3);
        assert_eq!(batch.total_count, 3);
        assert!(!batch.has_next_page);
        assert!(batch.next_page_token.is_none());
    }

    #[test]
    fn test_with_pagination() {
        let batch = ProviderBatch::with_pagination(
            vec![1, 2, 3],
            100,
            true,
            Some("cursor-123".to_string()),
        );

        assert_eq!(batch.len(), 3);
        assert_eq!(batch.total_count, 100);
        assert!(batch.has_next_page);
        assert!(!batch.is_last_page());
        assert_eq!(batch.next_page_token, Some("cursor-123".to_string()));
    }

    #[test]
    fn test_empty_batch() {
        let batch: ProviderBatch<i32> = ProviderBatch::empty();
        assert!(batch.is_empty());
        assert_eq!(batch.len(), 0);
        assert_eq!(batch.total_count, 0);
        assert!(batch.is_last_page());
    }

    #[test]
    fn test_map() {
        let batch = ProviderBatch::new(vec![1, 2, 3]);
        let mapped = batch.map(|x| x * 2);

        assert_eq!(mapped.items, vec![2, 4, 6]);
        assert_eq!(mapped.total_count, 3);
    }

    #[test]
    fn test_filter() {
        let batch = ProviderBatch::new(vec![1, 2, 3, 4, 5]);
        let filtered = batch.filter(|x| x % 2 == 0);

        assert_eq!(filtered.items, vec![2, 4]);
        assert_eq!(filtered.total_count, 2);
    }

    #[test]
    fn test_into_items() {
        let batch = ProviderBatch::new(vec![1, 2, 3]);
        let items = batch.into_items();

        assert_eq!(items, vec![1, 2, 3]);
    }
}
