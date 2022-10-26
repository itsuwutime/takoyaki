## Initializing a new takoyaki instance

Once you have got your Rust project setup and takoyaki installed, we are gonna create a empty takoyaki's core instance.

<br>

In your `main.rs` file, you will find a main function.

```rs
fn main() {
    println!("Hello, World!")
}
```

First, import `Takoyaki` from takoyaki SDK at the top of `main.rs`

```rs
use takoyaki::Takoyaki
```

Next, in the main function, create an instance of Takoyaki that you just imported

```rs
fn main() {
    let takoyaki = Takoyaki::<T , U>::default();
}
```

Woo, we have some generics being passed! The `T` generic is the type of data that it is going to handle (from either reqwest or cache) and the `U` generic is the type of config that it is going to handle. Lets create type generics for our GitHub plugin.

The type of data (`T`) will look like this:

```rs
use serde::{Deserialize , Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHub {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub contributions_collection: ContributionsCollection,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionsCollection {
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionCalendar {
    pub colors: Vec<String>,
    pub total_contributions: i64,
    pub weeks: Vec<Week>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Week {
    pub contribution_days: Vec<ContributionDay>,
    pub first_day: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionDay {
    pub color: String,
    pub contribution_count: i64,
    pub date: String,
    pub weekday: i64,
}
```

And the config type is gonna look like this:

```rs
#[derive(Deserialize , Debug , Default)]
pub struct Config {
    username: String,
    token: String
}
```

Perfect, we got both the types! Now, lets set these types in our takoyaki instance

```rs
fn main() {
    let mut takoyaki = Takoyaki::<GitHub , Config>::default();
}
```

Awesome! You have created a new instance of Takoyaki! 
