use std::collections::HashMap;

use clap::{App, Arg};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirmation, Editor, Select};
use glob::Pattern;

use super::command;
use super::issue;
use super::request;

use issue::Issue;
use request::Request;

const SELECTIONS: &[&str] = &["Open Issue", "Edit Issue", "Skip", "Exit"];
const ALL_FILES: &str = "*";

const OPEN: usize = 0;
const EDIT: usize = 1;
const SKIP: usize = 2;
const EXIT: usize = 3;

pub struct Args {
    pattern: Pattern,
    token: String,
}

impl Args {
    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn get_pattern(&self) -> &Pattern {
        &self.pattern
    }
}

pub fn init() -> Args {
    if !command::is_git_repo() {
        panic!("Must be in a git directory!");
    }

    let matches = App::new("todo-issue")
        .version("0.1")
        .author("Warren Galyen <wgalyen@hotmail.com>")
        .about("Converts TODO comments to GitHub issues")
        .arg(
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .value_name("PATTERN")
                .help("Sets a glob pattern to narrow search for TODO comments")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .value_name("TOKEN")
                .help("Sets the token for user")
                .takes_value(true)
        )
        .get_matches();

    let pattern_value = matches.value_of("pattern").unwrap_or(ALL_FILES);

    let pattern = match Pattern::new(pattern_value) {
        Ok(pattern) => pattern,
        Err(_) => Pattern::new(ALL_FILES).unwrap(),
    };

    let token = match matches.value_of("token") {
        Some(t) => t.to_string(),
        None => command::read_access_token(),
    };

    Args { pattern, token }
}

pub fn output_and_send_issues(
    request: &Request,
    map: &HashMap<String, Vec<Issue>>,
) {
    for (_file, issues) in map {
        for issue in issues {
            println!("\n{}", &issue.to_formatted_string());

            let prompt =
                format!("{}", style("What would you like to do?").italic())
                    .to_string();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(&prompt)
                .default(0)
                .items(&SELECTIONS[..])
                .interact()
                .unwrap();

            match selection {
                OPEN => open_issue(&request, &issue),
                EDIT => edit_and_open_issue(&request, &issue),
                SKIP => continue,
                EXIT => return,
                _ => (),
            }
        }
    }

    println!("All done!");
}

pub fn prompt_to_continue() -> bool {
    Confirmation::new()
        .with_text("Do you want to continue?")
        .interact()
        .expect("Failed to read confirmation")
}

fn open_issue(request: &Request, issue: &Issue) {
    match request.open_issue(issue) {
        Ok(()) => println!(
            "Successfully created issue with title: {}",
            issue.get_title()
        ),
        Err(e) => {
            let error_msg = format!(
                "Failed to open issue {}. Received error {}",
                issue.get_title(),
                e
            )
            .to_string();
            println!("{}", style(error_msg).yellow());
        }
    }
}

fn edit_and_open_issue(request: &Request, issue: &Issue) {
    match edit_issue(issue) {
        Some(issue) => open_issue(request, &issue),
        None => println!(
            "{}",
            style("Invalid format. Failed to create issue.").yellow()
        ),
    }
}

fn edit_issue(issue: &Issue) -> Option<Issue> {
    let result = Editor::new().edit(&issue.to_string()).unwrap();

    if let Some(input) = result {
        return Issue::from_string(input);
    } else {
        return None;
    }
}