use super::registry::CmdCtx;

/// Runs all tests in the `tests` module.
///
/// # Arguments
/// * `_ctx` - The command context, unused by this command.
///
/// # Example
/// ```rust,ignore
/// run().await;
/// ```
pub async fn run(_ctx: CmdCtx) {
    crate::tests::run_all_tests().await;
}
