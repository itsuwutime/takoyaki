use std::collections::HashMap;

use takoyaki_core::{plugin::Plugin, ready_state::ReadyState , reqwest};

use crate::types::Root;

pub struct GithubPlugin {

}

impl<'a> Plugin<'a , Root> for GithubPlugin {
    fn new() -> Self {
        Self {

        }
    }

    fn name(&self) -> &'a str {
        "github"
    }

    fn ready(&self) -> takoyaki_core::ready_state::ReadyState {
        let mut body = HashMap::new();

        body.insert("query", r#"query {
            user(login: "VoidCupboard") {
                name
                contributionsCollection {
                    contributionCalendar {
                        colors
                        totalContributions
                        weeks {
                            contributionDays {
                                color
                                contributionCount
                                date
                                weekday
                            }
                            firstDay
                        }
                    }
                }
            }
        }"#);

        ReadyState::from_reqwest(
            reqwest::Client::new()
                .post("https://api.github.com/graphql")
                .header("Authorization", "Bearer ")
                .body(serde_json::to_string(&body).unwrap())
        )
    }

    fn execute(&self , data: Root) {
        println!("{:?}" , data);
    }
}
