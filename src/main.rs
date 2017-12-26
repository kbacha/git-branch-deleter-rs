extern crate git2;
extern crate colored;
use git2::{Repository, BranchType};
use std::path::Path;
use std::io;
use std::io::Write;
use colored::*;

fn main() {
    let repo = Repository::open(&Path::new(".")).unwrap();
    let branches = repo.branches(Some(BranchType::Local)).unwrap();
    for mut m in branches {
        if let Ok((ref mut branch, BranchType::Local)) = m {
            let branch_name = branch.name().unwrap().unwrap().yellow();
            if branch.is_head() {
                println!("\nSkipping branch {} as it is currently {}",
                         branch_name,
                         "HEAD".green());
                continue;
            }

            'branch_questions: loop {
                print!("\nbranch: {}\nDelete? (y/n): ", branch_name);
                io::stdout().flush().unwrap();

                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                match buffer.trim().to_lowercase().as_str() {
                    "yes" | "y" => {
                        match branch.delete() {
                            Ok(_) => println!("{} has been {}", branch_name, "deleted".red()),
                            Err(e) => println!("An error occurred while deleting {}:\n{}", branch_name, e),
                        };
                        break 'branch_questions;
                    },
                    "no" | "n" | "" => {
                        println!("Skipping...");
                        break 'branch_questions;
                    },
                    other => println!("Did not recognize: '{}'", other.yellow()),
                }
            }
        }
    }
}
