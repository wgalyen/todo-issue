use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

use super::issue;
use super::request;
use issue::Issue;
use request::Request;

const TODO: &str = "TODO";

pub fn read_file(
    path: &str, 
    issues: &HashSet<String>,
    request: &Request,
) -> io::Result<()> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);

    let mut line_number = 0;
    for line_option in buffer.lines() {
        let line = line_option.unwrap();
        line_number += 1;

        if contains_todo(&line) {
            let title = extract_title(&line);
            let body = create_body(&line_number, path);

            if !issues.contains(title.as_str()) {
                let issue = Issue::new(&title, &body);
                let result = request.open_issue(issue);
                println!("{:?}", result);
            }
        }
    }

    Ok(())
}

fn contains_todo(line: &str) -> bool {
    // Look for C and Bash style comments
    let comment = match line.find("//") {
        Some(value) => Some(value),
        None => line.find("#"),
    };

    let todo = line.find(TODO);
    if comment.is_some() && todo.is_some() {
        let comment_index = comment.unwrap();
        let todo_index = todo.unwrap();
        return todo_index > comment_index;
    }

    false
}

fn extract_title(line: &str) -> String {
    let vec: Vec<&str> = line.split(TODO).collect();
    let after_todo = vec[1];
    let title = if after_todo.starts_with(":") {
        &after_todo[1..]
    } else {
        after_todo
    }
    .trim();

    title.to_string()
}

fn create_body(line_number: &u32, file_path: &str) -> String {
    format!(
        "Found a TODO comment on line {} of file {}",
        line_number, file_path
    )
    .to_string()
}
