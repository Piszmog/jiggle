use std::env;

use clap::Parser;

use crate::git::{checkout, has_conflicts, is_up_to_date, merge, pull, push};

mod git;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory of the repository.
    #[clap(short, long, value_parser, default_value = "./")]
    dir: String,

    /// The tree of branches to update.
    #[clap(short, long, value_parser)]
    tree: String,
}


fn main() {
    let args: Args = Args::parse();

    let mut previous_branch = "";
    args.tree.split('>').for_each(|branch| {
        println!("Checking out {:?}...", branch);
        checkout("/Users/randell/Documents/dev/sourcegraph", branch);

        println!("Pulling latest {:?}...", branch);
        pull("/Users/randell/Documents/dev/sourcegraph");

        if !previous_branch.is_empty() {
            let up_to_date = is_up_to_date("/Users/randell/Documents/dev/sourcegraph", previous_branch);

            if !up_to_date {
                let has_conflicts = has_conflicts("/Users/randell/Documents/dev/sourcegraph", "main");
                if !has_conflicts {
                    println!("Merging {:?} into {:?}...", previous_branch, branch);
                    merge("/Users/randell/Documents/dev/sourcegraph", previous_branch);

                    println!("Pushing updated {:?}...", branch);
                    push("/Users/randell/Documents/dev/sourcegraph");
                } else {
                    println!("There are merge conflicts between {:?} and {:?}. Manually resolve before running again.", previous_branch, branch)
                }
            } else {
                println!("{:?} is up-to-date with {:?}", branch, previous_branch)
            }
        }
        previous_branch = branch;
    });
    println!("All branches up-to-date!")
}
