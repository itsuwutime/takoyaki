![Demo for takoyaki](https://user-images.githubusercontent.com/115910279/197386883-cca681cd-3b89-43e3-89e4-817efa70dfbe.png)


# Takoyaki

Blazingly fast git contribution graph in your terminal 

# Features
:heavy_check_mark:  Customizable <br>
:heavy_check_mark:  Plugins to support a bunch of cloud based git repositories (like GitHub and GitLab) <br>
:heavy_check_mark:  Blazingly fast <br>

# Installation 

1. To install, run the following script:

```bash
curl https://takoyaki.vercel.app/install | sh
```

2. If you are on a Arch based distro, you can install `takoyaki` using `yay`

```bash
yay -S takoyaki
```

# Usage 

1. To install a plugin, run:

```
takoyaki plug <repo>
```

2. To delete a plugin, run:

```
takoyaki unplug <name>
```

3. To use a specific plugin, run:

```
takoyaki use <name>
```

4. To set a timeout for cache update, run:

```
takoyaki timeout <time_in_seconds>
```

# Plugin development guides

Currently, there is no docs for understanding how to build plugin, but soon there will be a comprehensive guide!

You add takoyaki's SDK by adding it as a dependency in your `Cargo.toml`

```toml
takoyaki = "1.0.1"
```

If you are brave enough, you can reference using the [github](https://github.com/kyeboard/takoyaki/tree/main/plugins/github) plugin or read the [generated docs](https://docs.rs/takoyaki/latest/takoyaki/)

# Contribution
I would love to have contributions to improve this tool even more ^-^
