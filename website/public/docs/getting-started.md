## Getting started with Takoyaki
<br>
Hi, welcome to Takoyaki's docs! Here you will find a comprehensive guide towards building your own plugin for Takoyaki! With that, you will also learn how to publish your awesome plugin to marketplace so that others can use your plugin.

Alright, so lets get started!

# Creating a Rust project

- First of all, initialize a new Rust project using the following command:

```
$ cargo init <name>
```

This will create a new Rust project with the following directory structure:

```
<name>/
├── Cargo.toml
└── src
    └── main.rs
```

- Next, add the takoyaki's SDK as a dependency

```
[dependencies]
takoyaki = "1.0.1"
```

- You might need to add `tokio` and `serde`

```
[dependencies]
tokio = { version = "1.21.2", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

Perfect! Thats pretty much it to get started. Run `cargo r` to install all the dependencies and run your Rust program
