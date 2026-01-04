//! Batch processing for bulk sync operations.

use crate::error::Result;
use std::future::Future;

/// Batch processor for handling large sync operations
pub struct BatchProcessor {
    batch_size: usize,
}

impl BatchProcessor {
    /// Create a new batch processor
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }

    /// Process items in batches
    pub async fn process<T, F, Fut>(&self, items: Vec<T>, processor: F) -> Result<BatchResult>
    where
        T: Clone,
        F: Fn(Vec<T>) -> Fut,
        Fut: Future<Output = Result<BatchResult>>,
    {
        let mut total_result = BatchResult::default();

        for chunk in items.chunks(self.batch_size) {
            let chunk_vec: Vec<T> = chunk.to_vec();
            let result = processor(chunk_vec).await?;
            total_result.merge(result);
        }

        Ok(total_result)
    }

    /// Get the batch size
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
}

impl BatchProcessor {
    /// Process items in batches with the items being cloneable
    pub async fn process_cloneable<T, F, Fut>(
        &self,
        items: &[T],
        processor: F,
    ) -> Result<BatchResult>
    where
        T: Clone,
        F: Fn(Vec<T>) -> Fut,
        Fut: Future<Output = Result<BatchResult>>,
    {
        let mut total_result = BatchResult::default();

        for chunk in items.chunks(self.batch_size) {
            let chunk_vec: Vec<T> = chunk.to_vec();
            let result = processor(chunk_vec).await?;
            total_result.merge(result);
        }

        Ok(total_result)
    }
}

/// Result of batch processing
#[derive(Debug, Default, Clone)]
pub struct BatchResult {
    /// Total items processed
    pub processed: u64,

    /// Items successfully completed
    pub succeeded: u64,

    /// Items that failed
    pub failed: u64,

    /// Error messages
    pub errors: Vec<String>,
}

impl BatchResult {
    /// Create a successful result for a single item
    pub fn success() -> Self {
        Self {
            processed: 1,
            succeeded: 1,
            failed: 0,
            errors: vec![],
        }
    }

    /// Create a failed result for a single item
    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            processed: 1,
            succeeded: 0,
            failed: 1,
            errors: vec![error.into()],
        }
    }

    /// Merge another result into this one
    pub fn merge(&mut self, other: BatchResult) {
        self.processed += other.processed;
        self.succeeded += other.succeeded;
        self.failed += other.failed;
        self.errors.extend(other.errors);
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.processed == 0 {
            100.0
        } else {
            (self.succeeded as f64 / self.processed as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_result_merge() {
        let mut result1 = BatchResult {
            processed: 10,
            succeeded: 8,
            failed: 2,
            errors: vec!["error1".to_string()],
        };

        let result2 = BatchResult {
            processed: 5,
            succeeded: 5,
            failed: 0,
            errors: vec![],
        };

        result1.merge(result2);

        assert_eq!(result1.processed, 15);
        assert_eq!(result1.succeeded, 13);
        assert_eq!(result1.failed, 2);
    }

    #[test]
    fn test_success_rate() {
        let result = BatchResult {
            processed: 100,
            succeeded: 95,
            failed: 5,
            errors: vec![],
        };

        assert_eq!(result.success_rate(), 95.0);
    }
}
