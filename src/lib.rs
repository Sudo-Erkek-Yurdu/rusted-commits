use colored::Colorize;
use std::collections::HashSet;
use std::process::Output;
use std::{process, process::Command};

pub struct GitGuard {
    cache: HashSet<String>,
}

impl GitGuard {
    pub fn new() -> Self {
        let cache = HashSet::new();
        Self { cache }
    }

    pub fn protect(&mut self, branch: &str) -> Self {
        let cache = &mut self.cache;

        if !cache.contains(branch) {
            cache.insert(branch.to_string());
        } else {
            println!(
                "{} {} {} {}",
                "TypeError:".bright_red(),
                "The".cyan(),
                branch.on_red(),
                "branch is already protected".cyan()
            );
            process::exit(1);
        }

        protect_branch(branch);
        Self {
            cache: cache.to_owned(),
        }
    }
}

fn protect_branch(branch_to_protect: &str) -> () {
    let current_branch: String = get_current_branch();

    if branch_to_protect == current_branch {
        println!(
            "{} {} {}",
            "You are not allowed commit changes to protected".cyan(),
            current_branch.on_red(),
            "branch directly.".cyan()
        );
    }
}

fn get_current_branch() -> String {
    let exec_output: Output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .unwrap_or_else(|err| {
            println!(
                "{}: {}",
                "Application error".cyan(),
                err.to_string().on_red()
            );
            process::exit(1);
        });

    let current_branch = String::from_utf8(exec_output.stdout).unwrap();

    current_branch.trim().to_string()
}
