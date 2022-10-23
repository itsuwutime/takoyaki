use std::collections::HashMap;

use serde::Deserialize;
use takoyaki_core::{plugin::Plugin, ready_state::ReadyState , reqwest , printable_grid::{PrintableGrid, Printable}};

use crate::types::Root;

pub struct GithubPlugin {

}

#[derive(Deserialize , Debug)]
pub struct Config {
    username: String,
    token: String
}

impl<'a> Plugin<'a , Root , Config> for GithubPlugin {
    fn new() -> Self {
        Self {

        }
    }

    fn name(&self) -> &'a str {
        "github"
    }

    fn ready(&self , config: Config) -> takoyaki_core::ready_state::ReadyState {
        let mut body = HashMap::new();

        body.insert("query", format!(r#"query {{
            user(login: "{}") {{
                name
                contributionsCollection {{
                    contributionCalendar {{
                        colors
                        totalContributions
                        weeks {{
                            contributionDays {{
                                color
                                contributionCount
                                date
                                weekday
                            }}
                            firstDay
                        }}
                    }}
                }}
            }}
        }}"# , config.username));

        ReadyState::from_reqwest(
            reqwest::Client::new()
                .post("https://api.github.com/graphql")
                .header("Authorization", format!("Bearer {}" , config.token))
                .body(serde_json::to_string(&body).unwrap())
        )
    }

    fn execute(&self , data: Root) -> PrintableGrid {
        let mut grid = PrintableGrid::new();
        let mut x = 0;
        let mut y = 0;

        for week in data.data.user.contributions_collection.contribution_calendar.weeks {
            for day in week.contribution_days {
                grid.insert(y as usize, x as usize, Printable { color: day.color , count: day.contribution_count as usize });

                y += 1;
            }

            x += 1;
            y = 0;
        }

        grid
    }
}
