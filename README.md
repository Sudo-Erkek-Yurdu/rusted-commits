# rusted-commits

A Rust pre-commit script to force the user to avoid committing changes directly to the protected branches in the local environment.

Adding new branches:

```rust
// src/main.rs

use git::GitGuard;

fn main() -> () {
  let mut git_guard = GitGuard::new();

  git_guard
      .protect("master")
      .protect("dev")
      .protect("prod")
      .protect("qa")
      .protect("uat"); // And many other branches you want to add
}
```

<hr>

Building: 

After configuring the protected fields, run `cargo build --release`
and move the executable binary in `/target/release/rusted-commits` to your git project.

<hr>

Using cargo to build with dynamic linking which will dramatically reduce the size of the binary, 
to build using dynamic link run
`cargo rustc --release -- -C prefer-dynamic`

If are on Linux and you want to reduce the size furthermore
you can also strip the binary of symbols using the `strip` command.

After building with dynamic link:
`strip target/release/rusted-commits`