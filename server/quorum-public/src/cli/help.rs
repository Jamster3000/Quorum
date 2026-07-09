//! Main cli file used for getting help with server commands

use colored::Colorize;

struct CommandEntry {
    command: &'static str,                           //full command
    summary: &'static str,                           //one line description
    description: &'static str,                       //the full command description
    usage: &'static str,                             //example usage
    requires_auth: bool,                             //whether login is required first
    requires_admin: bool,                            //whether admin privileges are required
    params: &'static [(&'static str, &'static str)], //parameter names and what they mean
}

const COMMANDS: &[CommandEntry] = &[
    // --help--
    CommandEntry {
        command: "help",
        summary: "Lists all available commands.",
        description: "Prints a summary of every available command, grouped by namespace.",
        usage: "help",
        requires_auth: false,
        requires_admin: false,
        params: &[],
    },
    CommandEntry {
        command: "help <command>",
        summary: "Shows detailed help for a specific command.",
        description: "Prints the full description, usage, parameters, and auth requirement for the given command.",
        usage: "help server:status",
        requires_auth: false,
        requires_admin: false,
        params: &[("command", "The command to look up, e.g. server:status")],
    },
    // --server--
    CommandEntry {
        command: "server:update-database",
        summary: "Updates server to use updated database.",
        description: "Reinitializes database for updating purposes, an identical process to startup, so that the server can use the most up to date database.",
        usage: "server:update-database",
        requires_auth: false,
        requires_admin: false,
        params: &[],
    },
    CommandEntry {
        command: "server:signup",
        summary: "Creates a new user account.",
        description: "Prompts for a username, password, and optional email address to create a new user account.",
        usage: "server:signup",
        requires_auth: false,
        requires_admin: false,
        params: &[],
    },
    CommandEntry {
        command: "server:login",
        summary: "Authenticates an operator session in the terminal.",
        description: "Prompts for a username and password and verifies them against the database. On success, grants access to write/destructive commands for the next 20 minutes of inactivity.",
        usage: "server:login",
        requires_auth: false,
        requires_admin: false,
        params: &[],
    },
    CommandEntry {
        command: "server:status",
        summary: "Shows uptime and basic server info.",
        description: "Prints how long the server has been running, what address it is listening on, and whether testing mode is active.",
        usage: "server:status",
        requires_auth: false,
        requires_admin: false,
        params: &[],
    },
    CommandEntry {
        command: "server:logout",
        summary: "Ends the current operator session.",
        description: "Clears the authenticated session immediately. Any subsequent write commands will require logging in again.",
        usage: "server:logout",
        requires_auth: true,
        requires_admin: false,
        params: &[],
    },
    CommandEntry {
        command: "server:shutdown",
        summary: "Gracefully shuts the server down.",
        description: "Signals the server to stop accepting new requests, waits for in-flight requests to complete, flushes logs, and exits cleanly.",
        usage: "server:shutdown",
        requires_auth: true,
        requires_admin: true,
        params: &[],
    },
    CommandEntry {
        command: "server:logs",
        summary: "Displays recent server logs.",
        description: "Prints logs in order from most recent to oldest, allowing for optional filter for last X days.",
        usage: "server:logs [days]",
        requires_auth: false,
        requires_admin: false,
        params: &[("days", "Optional number of days to display logs for")],
    },
    CommandEntry {
        command: "server:audit",
        summary: "Displays recent audit logs.",
        description: "Prints audit logs in order from most recent to oldest, allowing for optional filter for last X days.",
        usage: "server:audit [days]",
        requires_auth: false,
        requires_admin: false,
        params: &[("days", "Optional number of days to display audit logs for")],
    },
    // --db--
    CommandEntry {
        command: "db:stats",
        summary: "Displays database statistics.",
        description: "Prints the number of rows in each table, total row count, and estimated size of each table and database.",
        usage: "db:stats",
        requires_auth: false,
        requires_admin: false,
        params: &[],
    },
    CommandEntry {
        command: "db:table",
        summary: "Displays records from a specific table.",
        description: "Prints the records from the specified table, with an option to view a specific page of results.",
        usage: "db:table <name>, <page>",
        requires_auth: false,
        requires_admin: false,
        params: &[
            ("name", "The name of the table to display"),
            ("page", "The page of results to display"),
        ],
    },
    CommandEntry {
        command: "db:delete",
        summary: "Deletes the database and resets it to new",
        description: "Deletes the database and resets it to a new state. This action is irreversible and will remove all data.",
        usage: "db:delete",
        requires_auth: true,
        requires_admin: true,
        params: &[],
    },
    // --user--
    CommandEntry {
        command: "user:delete",
        summary: "Deletes a user account.",
        description: "Removes a user account from the database. This action is irreversible.",
        usage: "user:delete <id>",
        requires_auth: true,
        requires_admin: true,
        params: &[("id", "The ID of the user to delete")],
    },
    // --test--
    CommandEntry {
        command: "test:run",
        summary: "Runs functional tests on the server.",
        description: "Executes a suite of functional tests to verify that the server is operating correctly. This is only available if testing mode is enabled.",
        usage: "test:run",
        requires_auth: false,
        requires_admin: false,
        params: &[],
    },
    // --config--
    CommandEntry {
        command: "config:show",
        summary: "Displays the current server configuration.",
        description: "Prints the current server configuration settings.",
        usage: "config:show",
        requires_auth: true,
        requires_admin: true,
        params: &[],
    },
    CommandEntry {
        command: "config:<key>",
        summary: "Sets a configuration value.",
        description: "Updates a specific configuration setting in the server using the format `config:<key> <value>`. For example: `config:server_port 8080`. This action requires admin privileges.",
        usage: "config:<key> <value>",
        requires_auth: true,
        requires_admin: true,
        params: &[("value", "The new value for the configuration key")],
    },
];

/// Prints a summary of all available commands, grouped by namespace.
///
/// This is used when the user uses the `help` command with no parameters given.
pub fn print_all() {
    println!();
    println!("{}", "  Available Commands".cyan().bold());
    println!(
        "{}",
        "  ─────────────────────────────────────────────────────".dimmed()
    );

    let namespaces = ["help", "server", "db", "user", "test", "config"];

    for ns in namespaces {
        println!();
        println!("  {}", ns.white().bold());

        for entry in COMMANDS {
            let ns_prefix = format!("{}:", ns);
            let belongs = entry.command == ns
                || entry.command.starts_with(&ns_prefix)
                || (ns == "help" && entry.command.starts_with("help "));

            if belongs {
                let auth_marker = if entry.requires_auth {
                    " *".yellow().to_string()
                } else {
                    String::new()
                };
                println!(
                    "    {:<30} {}{}",
                    entry.command.green(),
                    entry.summary.dimmed(),
                    auth_marker
                );
            }
        }
    }

    println!();
    println!(
        "  {} {}",
        "*".yellow(),
        "marked commands require server:login before use.".dimmed()
    );
    println!(
        "  {}",
        "Run `help <command>` for detailed usage on any command.".dimmed()
    );
    println!();
}

/// Prints detailed help for a specific command, including its description, usage, parameters, and auth requirements.
///
/// This is used when the user uses the `help` command with a specific command name as a parameter.
pub fn print_command(command: &str) {
    let entry = COMMANDS.iter().find(|e| {
        e.command == command
            || e.command == format!("server:{}", command)
            || e.command == format!("help {}", command)
    });

    match entry {
        None => {
            println!("{} {}", "No help entry found for:".red(), command.white());
            println!("{}", "Run `help` to see all available commands.".dimmed());
        }
        Some(e) => {
            println!();
            println!("  {}", e.command.cyan().bold());
            println!(
                "{}",
                "  ─────────────────────────────────────────────────────".dimmed()
            );
            println!(
                "  {:<16} {}",
                "Description:".white(),
                e.description.dimmed()
            );
            println!("  {:<16} {}", "Usage:".white(), e.usage.green());
            println!(
                "  {:<16} {}",
                "Auth required:".white(),
                if e.requires_auth {
                    "yes".yellow().to_string()
                } else {
                    "no".dimmed().to_string()
                }
            );

            println!(
                "  {:<16} {}",
                "Admin permissions required:".white(),
                if e.requires_admin {
                    "yes".yellow().to_string()
                } else {
                    "no".dimmed().to_string()
                }
            );

            if !e.params.is_empty() {
                println!("  {}", "Parameters:".white());
                for (name, desc) in e.params {
                    println!("    {:<16} {}", name.green(), desc.dimmed());
                }
            }

            println!();
        }
    }
}
