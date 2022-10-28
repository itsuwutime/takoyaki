## The ready function

The `ready()` function requires you to get your data ready to be printed in the terminal. The format of the function is something like this:

```rs
fn ready(&self , cache: Cache, config: Config) -> ReadyState
```

Takoyaki sends in a bunch of params to ease your work. The first `cache` allows to retrieve any available cache so that you can give a blazingly fast response when the plugin is launched. The second one, `config`, allows you to access the data entered during the installation of the plugin (like username or token).

<br>

It expects you to return a ReadyState object which can be built in two ways:

```rs
fn from_cache(cache: Cache) -> ReadyState
```

This function allows you to build from the existing cache.

<br>

```rs
fn from_reqwest(client: RequestBuilder) -> ReadyState
```

This function allows you to build from a reqwest client and the data is parsed and sent to be processed.

