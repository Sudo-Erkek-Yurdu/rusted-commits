use git::GitGuard;

fn main() -> () {
    let mut git_guard = GitGuard::new();

    git_guard.protect("master").protect("dev");
}
