mod types;

use std::collections::HashMap;
use takoyaki::{Takoyaki, ReadyState, PrintableGrid, Cache , Printable};

#[tokio::main]
async fn main() {
    let mut takoyaki = Takoyaki::<types::Root>::new("github");

    takoyaki.set_ready(Box::new(|| {
        let cache = Cache::new("github").unwrap();

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
        }}"# , "ThePrimeagen"));

        ReadyState::from_reqwest(
            reqwest::Client::new()
                .post("https://api.github.com/graphql")
                .header("Authorization", format!("Bearer {}" , ""))
                .body(serde_json::to_string(&body).unwrap())
        )

    }));

    takoyaki.set_execute(Box::new(|data| {
        let mut grid = PrintableGrid::new();

        let mut x = 0;
        let mut y = 0;

        for week in data.data.user.contributions_collection.contribution_calendar.weeks {
            for day in week.contribution_days {
                grid.insert_at(x, y, Printable { color: day.color , contributions: day.contribution_count } );

                x += 1;
            }

            x = 0;
            y += 1;
        };

        grid
    }));

    takoyaki.start().await.unwrap();
}
