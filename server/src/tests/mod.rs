mod functional;

use crate::startup;
use colored::*;
use futures::future::BoxFuture;

type TestFn = fn() -> BoxFuture<'static, Result<TestResult, String>>;

pub struct TestResult {
    pub endpoint_time: u128,
}

pub async fn run_all_tests() {
    println!("\n{}", "Running Tests...".yellow().bold());

    let tests: Vec<(&str, TestFn)> = vec![
        ("Auth signup with email + password test", || {
            Box::pin(functional::auth_tests::test_signup_email())
        }),
        ("Auth signup with username + password test", || {
            Box::pin(functional::auth_tests::test_signup_username())
        }),
        ("Auth login with email test", || {
            Box::pin(functional::auth_tests::test_login_email())
        }),
        ("Auth login with username test", || {
            Box::pin(functional::auth_tests::test_login_username())
        }),
        ("Auth refresh token test", || {
            Box::pin(functional::auth_tests::test_refresh_token())
        }),
        ("Auth logout test", || {
            Box::pin(functional::auth_tests::test_logout())
        }),
        ("Auth delete user account with email test", || {
            Box::pin(functional::auth_tests::test_delete_user_account_email())
        }),
        ("Auth delete user account with username test", || {
            Box::pin(functional::auth_tests::test_delete_user_account_username())
        }),
    ];

    let mut failed_tests = Vec::new();

    for (i, (test_name, test_fn)) in tests.iter().enumerate() {
        let timer = startup::create_timer();
        let is_last = i == tests.len() - 1;

        match test_fn().await {
            Ok(result) => {
                if is_last {
                    startup::print_final_step(test_name, true, result.endpoint_time);
                } else {
                    startup::print_step(test_name, true, result.endpoint_time);
                }
            }
            Err(e) => {
                let elapsed = startup::elapsed_ms(timer);
                if is_last {
                    startup::print_final_step(test_name, false, elapsed);
                } else {
                    startup::print_step(test_name, false, elapsed);
                }
                failed_tests.push((test_name.to_string(), e));
            }
        }
    }

    if !failed_tests.is_empty() {
        println!(
            "\n{}",
            format!("{} test(s) failed:", failed_tests.len())
                .red()
                .bold()
        );
        for (name, error) in &failed_tests {
            println!("{}", format!("  ✗ {}", name).red());
            let error_lines: Vec<&str> = error.lines().collect();
            for (idx, line) in error_lines.iter().enumerate() {
                if idx == error_lines.len() - 1 {
                    println!("    {} {}", "└─".red(), line.dimmed());
                } else {
                    println!("    {} {}", "├─".red(), line.dimmed());
                }
            }
        }
    }
}
