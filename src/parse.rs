use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

use super::issue;
use issue::Issue;

const TODO: &str = "TODO";

pub fn populate_map(
    files: &Vec<String>,
    issues: &HashSet<String>,
    pattern: &glob::Pattern,
) -> HashMap<String, Vec<Issue>> {
    let mut file_to_issues = HashMap::new();

    for path in files {
        if pattern.matches(&path) {
            let result = find_issues(&path, &issues);
            if let Ok(vector) = result {
                file_to_issues.insert(path.clone(), vector);
            }
        }
    }

    file_to_issues
}

pub fn count_issues(map: &HashMap<String, Vec<issue::Issue>>) -> usize {
    let mut total = 0;
    for (file, issues) in map {
        let num_issues = issues.len();
        if num_issues > 0 {
            println!("Found {} issues in file {}", num_issues, file);
            total += num_issues;
        }
    }

    println!("Found {} issues total.", total);
    return total;
}

fn find_issues(
    path: &str, 
    prev_issues: &HashSet<String>,
) -> io::Result<Vec<Issue>> {
    //! Reads every line in a file for a "todo" comment, creating an Issue
    //! object for each one with the parsed title and description.
    //! 
    //! Returns an IO result containing a vector of Issues if successful.
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let mut issues_in_file = Vec::new();

    let mut line_number = 0;
    for line_option in buffer.lines() {
        let line = line_option.unwrap();
        line_number += 1;

        if contains_todo(&line) {
            let title = extract_title(&line);
            let body = create_body(&line_number, path);

            if !prev_issues.contains(title.as_str()) {
                let issue = Issue::new(title, body);
                issues_in_file.push(issue);
            }
        }
    }

    Ok(issues_in_file)
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
    //! Parses a line containing a todo comment and returns the
    //! remainder of the String after "todo" to be used as the title of a
    //! new GitHub issue.
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
    //! Creates a generic description for a new GitHub issue
    //! based on a "todo" comment.
    format!(
        "Found a TODO comment on line {} of file {}",
        line_number, file_path
    )
    .to_string()
}
