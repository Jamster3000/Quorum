/// Runs all tests in the `tests` module.
///
/// # Example
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     run().await;
/// }
/// ```
pub async fn run() {
    crate::tests::run_all_tests().await;
}