# Release Instructions

- Change `Cargo.toml` version
- Run `cargo run -p ci` and ensure CI passes
- Ensure `CHANGELOG.md` has all relevant changes from git history
- Update `README.md`
  - Change `#[dependencies]` to correct version
  - Update "ENet Versions" table
- Commit to git with commit message being the version number ("0.4.0")
- Push and check that CI passes
- Run `cargo publish`
- Create the GitHub release with the previous commit hash
