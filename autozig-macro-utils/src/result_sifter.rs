//! Result aggregation utilities for collecting multiple Results.
//!
//! The `ResultSifter` allows you to collect multiple operations that return `Result`,
//! accumulating all errors while still collecting successful values.

use syn::{Error, Result};

/// A utility for sifting through multiple Results, collecting both successes and errors.
///
/// This is useful in procedural macros when you want to validate multiple items
/// and report all errors at once, rather than stopping at the first error.
///
/// # Examples
///
/// ```ignore
/// let mut sifter = ResultSifter::new();
///
/// for field in fields {
///     sifter.push(validate_field(field));
/// }
///
/// let values = sifter.finish()?;
/// ```
#[derive(Default)]
pub struct ResultSifter<T> {
    values: Vec<T>,
    errors: Vec<Error>,
}

impl<T> ResultSifter<T> {
    /// Creates a new empty `ResultSifter`.
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Adds a `Result` to the sifter.
    ///
    /// If the result is `Ok`, the value is stored.
    /// If the result is `Err`, the error is accumulated.
    pub fn push(&mut self, result: Result<T>) {
        match result {
            Ok(value) => self.values.push(value),
            Err(error) => self.errors.push(error),
        }
    }

    /// Adds multiple results to the sifter.
    pub fn extend(&mut self, results: impl IntoIterator<Item = Result<T>>) {
        for result in results {
            self.push(result);
        }
    }

    /// Returns true if any errors have been accumulated.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns the number of errors accumulated.
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Returns the number of successful values collected.
    pub fn value_count(&self) -> usize {
        self.values.len()
    }

    /// Consumes the sifter and returns the collected values if there are no errors.
    ///
    /// If there are any errors, they are combined into a single error and returned.
    pub fn finish(self) -> Result<Vec<T>> {
        if self.errors.is_empty() {
            Ok(self.values)
        } else {
            Err(combine_errors(self.errors))
        }
    }

    /// Consumes the sifter and returns both values and errors.
    ///
    /// This is useful when you want to handle partial success.
    pub fn finish_split(self) -> (Vec<T>, Vec<Error>) {
        (self.values, self.errors)
    }

    /// Returns a reference to the collected values so far.
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// Returns a reference to the accumulated errors.
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    /// Clears all values and errors.
    pub fn clear(&mut self) {
        self.values.clear();
        self.errors.clear();
    }
}

/// Combines multiple errors into a single error.
///
/// The errors are combined by chaining their messages together.
pub fn combine_errors(errors: Vec<Error>) -> Error {
    let mut iter = errors.into_iter();
    let mut combined = iter.next().expect("combine_errors called with empty vector");
    
    for error in iter {
        combined.combine(error);
    }
    
    combined
}

/// Creates a `ResultSifter` from an iterator of results.
impl<T> FromIterator<Result<T>> for ResultSifter<T> {
    fn from_iter<I: IntoIterator<Item = Result<T>>>(iter: I) -> Self {
        let mut sifter = Self::new();
        sifter.extend(iter);
        sifter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_result_sifter_all_ok() {
        let mut sifter = ResultSifter::new();
        sifter.push(Ok(1));
        sifter.push(Ok(2));
        sifter.push(Ok(3));

        assert!(!sifter.has_errors());
        assert_eq!(sifter.value_count(), 3);
        assert_eq!(sifter.error_count(), 0);

        let values = sifter.finish().unwrap();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_result_sifter_all_err() {
        let mut sifter: ResultSifter<i32> = ResultSifter::new();
        sifter.push(Err(Error::new(proc_macro2::Span::call_site(), "error 1")));
        sifter.push(Err(Error::new(proc_macro2::Span::call_site(), "error 2")));

        assert!(sifter.has_errors());
        assert_eq!(sifter.value_count(), 0);
        assert_eq!(sifter.error_count(), 2);

        assert!(sifter.finish().is_err());
    }

    #[test]
    fn test_result_sifter_mixed() {
        let mut sifter = ResultSifter::new();
        sifter.push(Ok(1));
        sifter.push(Err(Error::new(proc_macro2::Span::call_site(), "error")));
        sifter.push(Ok(2));

        assert!(sifter.has_errors());
        assert_eq!(sifter.value_count(), 2);
        assert_eq!(sifter.error_count(), 1);

        let (values, errors) = sifter.finish_split();
        assert_eq!(values, vec![1, 2]);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_result_sifter_extend() {
        let mut sifter = ResultSifter::new();
        let results = vec![Ok(1), Ok(2), Ok(3)];
        sifter.extend(results);

        assert_eq!(sifter.value_count(), 3);
        assert!(!sifter.has_errors());
    }

    #[test]
    fn test_result_sifter_from_iterator() {
        let results = vec![Ok(1), Ok(2), Err(Error::new(proc_macro2::Span::call_site(), "error"))];
        let sifter: ResultSifter<i32> = results.into_iter().collect();

        assert_eq!(sifter.value_count(), 2);
        assert_eq!(sifter.error_count(), 1);
    }

    #[test]
    fn test_result_sifter_clear() {
        let mut sifter = ResultSifter::new();
        sifter.push(Ok(1));
        sifter.push(Err(Error::new(proc_macro2::Span::call_site(), "error")));

        sifter.clear();
        assert_eq!(sifter.value_count(), 0);
        assert_eq!(sifter.error_count(), 0);
    }
}