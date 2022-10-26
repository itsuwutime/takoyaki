## Creating your first plugin

Awesome, you made this far! Now that we have a instance of Takoyaki running, the next to do is creating a plugin that is executable by takoyaki. For this, you must have a struct that implements the `Plugin` trait.

Let's say that you have this struct named `GitHubPlugin`

```rs
struct GitHubPlugin {
    
}
```

Next, we are gonna implement the `Plugin` trait. Before that, at the top, import the trait

```rs
use takoyaki::Plugin;
```

Last, implement that trait for the struct

```rs
use takoyaki::{ReadyState , Cache};

impl<'a> Plugin<'a , GitHub, Config> for GitHubPlugin {
    fn new() -> Self {

    }

    fn name(&self) -> &'a str {

    }

    fn ready(&self , config: Config , cache: Cache) -> ReadyState {
        
    }

    fn execute(&self , data: GitHub) {

    }
}
```

Wooooooooooooooooooo! We also have a bunch of type generics! The first one is just a lifetime specifier, the second one being the type of data that it is going to handle and the last but not the least being the config type.

Each plugin has four function that they handle. They are:

<br>

1. `new()` - This function returns a new instance of itself
2. `name()` - This function returns the name of the plugin. It is used to assign and get the cache
3. `ready()` - This function returns the data that will be printed. It allows you to retrieve data in two ways, by `cache` and `reqwest`
4. `execute()` - This function will get the data as the type of `T` and return a `PrintableGrid` that will be futher printed by `takoyaki`


