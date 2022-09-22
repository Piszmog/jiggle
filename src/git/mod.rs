use std::process::{Child, Command};

pub fn checkout(branch: String) -> Child {
    Command::new("git").arg("checkout").arg(branch).spawn().expect("failed to checkout branch")
}

pub fn merge(root_branch: String) -> Child {
    Command::new("git").arg("merge").arg(root_branch).arg("--no-edit").spawn().expect("failed to merge root")
}

pub fn pull() -> Child {
    Command::new("git").arg("pull").spawn().expect("failed to pull latest")
}

pub fn push() -> Child {
    Command::new("git").arg("push").spawn().expect("failed to push updated code")
}
