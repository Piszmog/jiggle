use std::process::{Command, Stdio};

/// Determines if the specified branch exists.
pub fn branch_exists(repo_dir: &str, branch: &str) -> bool {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("show-branch")
        .arg(branch)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to check if branch exists")
        .wait_with_output()
        .unwrap();
    String::from_utf8(output.stderr).unwrap().is_empty()
}

/// Checkouts the specified branch.
pub fn checkout(repo_dir: &str, branch: &str) {
    Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("checkout")
        .arg(branch)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to checkout branch")
        .wait()
        .unwrap();
}

/// Determines if the current checked out branch will have any conflicts when merging the
/// specified branch.
pub fn has_conflicts(repo_dir: &str, from_branch: &str) -> bool {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("merge")
        .arg(from_branch)
        .arg("--no-ff")
        .arg("--no-commit")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to check if there are conflicts")
        .wait_with_output()
        .unwrap();

    let message = String::from_utf8(output.stdout).unwrap();
    let messages = message
        .split('\n')
        .map(|s| s.trim());

    // Abort the merge to get to a clean state.
    abort_merge(repo_dir);

    for message in messages {
        if message == "Automatic merge failed; fix conflicts and then commit the result." {
            return true;
        }
    }
    false
}

fn abort_merge(repo_dir: &str) {
    Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("merge")
        .arg("--abort")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to abort merge")
        .wait()
        .unwrap();
}

/// Determines if the current checked out branch has the latest code from the specified branch.
pub fn is_up_to_date(repo_dir: &str, from_branch: &str) -> bool {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("branch")
        .arg("--contains")
        .arg(from_branch)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to check if branch is up-to-date")
        .wait_with_output()
        .unwrap();

    let message = String::from_utf8(output.stdout).unwrap();
    let branches = message
        .split('\n')
        .map(|s| s.trim());

    // Determine which branch we are currently sitting on.
    let current_branch = get_current_branch(repo_dir);

    // If the current branch has the latest from_branch, we will see the current branch with a '*' prefix.
    // e.g. "* my-little-branch"
    let up_to_date_value = format!("* {}", current_branch);

    let mut up_to_date = false;
    for branch in branches {
        if branch == current_branch {
            break;
        } else if branch == up_to_date_value {
            up_to_date = true;
            break;
        }
    }
    up_to_date
}

fn get_current_branch(repo_dir: &str) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("branch")
        .arg("--show-current")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to get current branch")
        .wait_with_output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap().strip_suffix('\n').unwrap().to_string()
}

/// Merges the specified branch into the current checked out branch.
pub fn merge(repo_dir: &str, from_branch: &str) {
    Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("merge")
        .arg(from_branch)
        .arg("--no-edit")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to merge root")
        .wait()
        .unwrap();
}

/// Pulls the latest code into the current checked out branch.
pub fn pull(repo_dir: &str) {
    Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("pull")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to pull latest")
        .wait()
        .unwrap();
}

/// Pushes the current checked out branch to remote.
pub fn push(repo_dir: &str) {
    Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("push")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to push updated code")
        .wait()
        .unwrap();
}
