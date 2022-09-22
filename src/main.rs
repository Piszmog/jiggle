use std::env;

use crate::git::{checkout, merge, pull, push};

mod git;

fn main() {
    let args: Vec<String> = env::args().collect();
    let branches = args[1].split('>');

    let mut previous_branch = "";
    branches.for_each(|branch| {
        println!("Checking out {:?}...", branch);
        checkout(branch.to_string()).wait().unwrap();

        println!("Pulling latest {:?}...", branch);
        pull().wait().unwrap();

        if !previous_branch.is_empty() {
            println!("Merging {:?} into {:?}...", previous_branch, branch);
            merge(previous_branch.to_string()).wait().unwrap();

            println!("Pushing updated {:?}...", branch);
            push().wait().unwrap();
        }
        previous_branch = branch;
    });
    println!("All branches up-to-date!")
}
