cargo xtask
==============

[cargo-xtask](https://github.com/matklad/cargo-xtask) is way to add free-form automation to a Rust project, a-la make, npm run or bespoke bash scripts.
The two distinguishing features of xtask are:

- It doesn't require any other binaries besides cargo and rustc, it fully bootstraps from them
- Unlike bash, it can more easily be cross-platform, as it doesn't use the shell.

# Standard tasks

* `--help`: list all available tasks, and output as following:

```
Tasks:
  dist - builds application and man pages
```

# Tooling

- [devx](https://github.com/elastio/devx): collection of useful utilities (spawning processes, git pre-commit hooks, etc.)
- [xshell](https://github.com/matklad/xshell): ergonomic "bash" scripting in Rust
- [duct](https://github.com/oconnor663/duct.rs): a library for running child processes with support for pipelines and IO redirection
- [xtaskops](https://github.com/jondot/xtaskops): Goodies for working with the xtask concept in Rust
- [cli-xtask](https://github.com/gifnksm/cli-xtask): A collection of utility functions and command line interfaces for cargo-xtask

# References

* cargo xtask: https://github.com/matklad/cargo-xtask
* Running Rust Tasks With “xtask” and “xtaskops”: https://betterprogramming.pub/running-rust-tasks-with-xtask-and-xtaskops-a2193e67dc25
* Cargo Configuration: https://doc.rust-lang.org/cargo/reference/config.html
