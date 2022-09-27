use clap::Parser;

use crate::git::{branch_exists, checkout, has_conflicts, is_up_to_date, merge, pull, push};

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

    let repo_dir = args.dir.as_str();
    let mut previous_branch = "";
    for branch in args.tree.split('>') {
        let exists = branch_exists(repo_dir, branch);
        if !exists {
            println!("Branch {:?} does not exist. Exiting...", branch);
            return;
        }

        println!("Checking out {:?}...", branch);
        checkout(repo_dir, branch);

        println!("Pulling latest {:?}...", branch);
        pull(repo_dir);

        if !previous_branch.is_empty() {
            let up_to_date = is_up_to_date(repo_dir, previous_branch);

            if !up_to_date {
                let has_conflicts = has_conflicts(repo_dir, previous_branch);
                if !has_conflicts {
                    println!("Merging {:?} into {:?}...", previous_branch, branch);
                    merge(repo_dir, previous_branch);

                    println!("Pushing updated {:?}...", branch);
                    push(repo_dir);
                } else {
                    println!("There are merge conflicts between {:?} and {:?}. Manually resolve before running again.", previous_branch, branch);
                    return;
                }
            } else {
                println!("{:?} is up-to-date with {:?}", branch, previous_branch);
            }
        }
        previous_branch = branch;
    };
    println!("All branches up-to-date!");
}
