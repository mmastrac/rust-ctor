workflow "New workflow" {
  on = "push"
  resolves = ["icepuma/rust-action@master"]
}

action "icepuma/rust-action@master" {
  uses = "icepuma/rust-action@master"
  args = "cargo install cargo-expand && cargo test"
}
