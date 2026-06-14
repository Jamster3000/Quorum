use colored::Colorize;

pub async fn run() {
	println!("{}", "  Running tests...".yellow().bold());
	crate::tests::run_all_tests().await;
}