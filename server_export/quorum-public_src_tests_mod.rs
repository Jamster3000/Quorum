mod common;
mod functional;
mod robust;

use colored::*;
use futures::future::BoxFuture;
use quorum_core::startup;
use std::time::Duration;

// 1. Define Type Aliases to dramatically simplify the code
type FunctionalTestFn = fn() -> BoxFuture<'static, Result<TestResult, String>>;
type RobustnessTestFn = fn() -> BoxFuture<'static, Result<RobustnessTestResult, String>>;

pub struct TestResult {
    pub endpoint_time: Duration,
}

pub struct RobustnessTestResult {
    pub endpoint_time: Duration,
}

pub async fn run_all_tests() {
    println!("\n{}", "Running Tests...".yellow().bold());

    run_functional_tests().await;
    run_robust_tests().await;
}

async fn run_functional_tests() {
    println!("\n{}", "Running Functional Tests...".cyan().bold());

    let tests: Vec<(&str, FunctionalTestFn)> = vec![
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
        ("Auth update user profile test", || {
            Box::pin(functional::auth_tests::test_update_user_profile())
        }),
    ];

    run_test_suite(&tests).await;
}

async fn run_robust_tests() {
    println!("\n{}", "Running Robustness Tests...".magenta().bold());

    let tests: Vec<(&str, RobustnessTestFn)> = vec![
        ("Signup with short username", || {
            Box::pin(robust::auth_tests::test_signup_short_username())
        }),
        ("Signup with long username", || {
            Box::pin(robust::auth_tests::test_signup_long_username())
        }),
        ("Signup with empty password", || {
            Box::pin(robust::auth_tests::test_signup_empty_password())
        }),
        ("Signup with short password", || {
            Box::pin(robust::auth_tests::test_signup_short_password())
        }),
        ("Signup with long password", || {
            Box::pin(robust::auth_tests::test_signup_long_password())
        }),
        ("Signup with invalid email", || {
            Box::pin(robust::auth_tests::test_signup_invalid_email())
        }),
        ("Signup with duplicate username", || {
            Box::pin(robust::auth_tests::test_signup_duplicate_username())
        }),
        ("Login with wrong password", || {
            Box::pin(robust::auth_tests::test_login_wrong_password())
        }),
        ("Login with nonexistent user", || {
            Box::pin(robust::auth_tests::test_login_nonexistent_user())
        }),
        ("Login with empty username", || {
            Box::pin(robust::auth_tests::test_login_empty_username())
        }),
        ("Refresh with invalid token", || {
            Box::pin(robust::auth_tests::test_refresh_invalid_token())
        }),
        ("Refresh with empty token", || {
            Box::pin(robust::auth_tests::test_refresh_empty_token())
        }),
        ("Delete with wrong password", || {
            Box::pin(robust::auth_tests::test_delete_wrong_password())
        }),
        ("Get user data with wrong password", || {
            Box::pin(robust::auth_tests::test_get_user_data_wrong_password())
        }),
        ("Logout with invalid token", || {
            Box::pin(robust::auth_tests::test_logout_invalid_token())
        }),
        ("Updating profile username with an empty field", || {
            Box::pin(robust::auth_tests::test_update_profile_empty_username())
        }),
    ];

    run_robustness_suite(&tests).await;
}

async fn run_test_suite(tests: &[(&str, FunctionalTestFn)]) {
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
                let elapsed = startup::elapsed(timer);
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

// Cleaned up function signature
async fn run_robustness_suite(tests: &[(&str, RobustnessTestFn)]) {
    let mut issue_tests = Vec::new();

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
                let elapsed = startup::elapsed(timer);
                if is_last {
                    startup::print_final_step(test_name, false, elapsed);
                } else {
                    startup::print_step(test_name, false, elapsed);
                }
                issue_tests.push((test_name.to_string(), e));
            }
        }
    }

    if !issue_tests.is_empty() {
        println!(
            "\n{}",
            format!("{} test(s) with unexpected behavior:", issue_tests.len())
                .yellow()
                .bold()
        );
        for (name, error) in &issue_tests {
            println!("{}", format!("  ⚠ {}", name).yellow());
            let error_lines: Vec<&str> = error.lines().collect();
            for (idx, line) in error_lines.iter().enumerate() {
                if idx == error_lines.len() - 1 {
                    println!("    {} {}", "└─".yellow(), line.dimmed());
                } else {
                    println!("    {} {}", "├─".yellow(), line.dimmed());
                }
            }
        }
    }
}
