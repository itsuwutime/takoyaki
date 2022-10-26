## Getting started with Takoyaki

Hi, welcome to Takoyaki's docs! Here you will find a comprehensive guide towards building your own plugin for Takoyaki! With that, you will also learn how to publish your awesome plugin to marketplace so that others can use your plugin.

In this tutorial, we are gonna be building a GitHub plugin from scratch so that you get the ropes!

# Creating a Rust project

First of all, initialize a new Rust project using the following command:

```shell
$ cargo init <name>
```

This will create a new Rust project with the following directory structure:

```txt
<name>/
├── Cargo.toml
└── src
    └── main.rs
```

# Adding takoyaki's SDK as a dependency

```toml
[dependencies]
takoyaki = "1.0.1"
```

- You might need to add `tokio` and `serde`

```toml
[dependencies]
tokio = { version = "1.21.2", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

Perfect! Thats pretty much it to get started. Run `cargo r` to install all the dependencies and run your Rust program
