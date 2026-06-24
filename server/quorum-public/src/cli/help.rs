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
    // --server--
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
        command: "server:make-admin",
        summary: "Promotes a user to an administrator.",
        description: "Converts a regular user into an administrator with elevated privileges.",
        usage: "server:make-admin <username>",
        requires_auth: false,
        requires_admin: false,
        params: &[("username", "The username of the user to promote")],
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

    let namespaces = ["server", "test"];

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
