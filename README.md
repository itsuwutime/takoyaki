<img src="https://img.shields.io/badge/Rust-2E3440?style=for-the-badge&logo=rust&logoColor=white"> <img src="https://img.shields.io/crates/d/takoyaki?style=for-the-badge"> <img src="https://img.shields.io/github/issues/kyeboard/takoyaki?style=for-the-badge" > <img src="https://img.shields.io/crates/l/takoyaki?style=for-the-badge"> <img src="https://img.shields.io/crates/v/takoyaki?style=for-the-badge" />

# Takoyaki

Blazingly fast git contribution graph in your terminal 

# Features
:heavy_check_mark:  Customizable <br>
:heavy_check_mark:  Plugins to support a bunch of cloud based git repositories (like GitHub and GitLab) <br>
:heavy_check_mark:  Blazingly fast <br>

# Installation 

To install, run the following script:

```bash
curl https://usetakoyaki.vercel.app/install.sh | sh
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

# Notes
Fetching everytime might take your terminal to open super late, which sucks. So, Tokayaki uses caching to cache the response so that you get a blazingly fast response on opening your terminal. This cache is updated every hour using the service or if you wanna force update it, run `takoyaki clean`

# Plugin development guides

Currently, there is no docs for understanding how to build plugin, but soon there will be a comprehensive guide! If you are brave enough, you can reference using the [github](https://github.com/kyeboard/takoyaki/tree/main/plugins/github) plugin or read the [generated docs](https://docs.rs/takoyaki/latest/takoyaki/)

# Contribution
I would love to have contributions to improve this tool even more ^-^
