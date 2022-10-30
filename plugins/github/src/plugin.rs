use std::collections::HashMap;
use serde::Deserialize;
use takoyaki::{Plugin, ReadyState , reqwest , PrintableGrid, Printable , Cache , State};

use crate::types::Root;

pub struct GithubPlugin;

#[derive(Deserialize , Debug , Default)]
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

    fn ready(&self , config: Config , cache: Cache) -> ReadyState {
        if cache.exists() {
            return ReadyState::from_cache(cache)
        }

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
                grid.insert(
                    x as usize, 
                    y as usize, 
                    Printable { 
                        color: day.color, 
                        count: day.contribution_count as usize, 
                        state: State::Some 
                    }
                );

                x += 1;
            }

            y += 1;
            x = 0;
        }

        grid
    }
}

