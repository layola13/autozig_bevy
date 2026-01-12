//! Task type for async operations

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A spawned task that can be awaited
pub struct Task<T> {
    inner: Pin<Box<dyn Future<Output = T> + Send>>,
}

impl<T> Task<T> {
    /// Create a new task from a future
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        Self {
            inner: Box::pin(future),
        }
    }

    /// Detach the task, allowing it to run in the background
    pub fn detach(self) {
        // Task will continue running until completion
        drop(self);
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(cx)
    }
}