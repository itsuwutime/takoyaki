<img src="https://user-images.githubusercontent.com/115910279/198840631-536c0f35-01db-463f-8b3a-109f6747b251.png">

<img src="https://img.shields.io/badge/Rust-2E3440?style=for-the-badge&logo=rust&logoColor=white"> <img src="https://img.shields.io/crates/d/takoyaki?style=for-the-badge"> <img src="https://img.shields.io/github/issues/kyeboard/takoyaki?style=for-the-badge" > <img src="https://img.shields.io/crates/l/takoyaki?style=for-the-badge"> <img src="https://img.shields.io/crates/v/takoyaki?style=for-the-badge" />

# Takoyaki

Blazingly fast git contribution graph in your terminal 

# Features
:heavy_check_mark:  Customizable <br>
:heavy_check_mark:  Plugins to support a bunch of cloud based git repositories (like GitHub and GitLab) <br>
:heavy_check_mark:  Blazingly fast <br>
:heavy_check_mark:  Cross platform support <br>
:heavy_check_mark:  Open source <br>

# Installation 

You can find the installation guide for your OS here - https://takoyaki.keyboard.me/install

# Usage 

1. To install a plugin (you can get all the plugins here - https://takoyaki.kyeboard.me/marketplace), run:

```
takoyaki plug <name>
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
I would be glad to have contributions for the plugins! Read a comprehensive guide on how to create a plugin here - https://takoyaki.kyeboard.me/documentation

# Contribution
I would love to have contributions to improve this tool even more ^-^
