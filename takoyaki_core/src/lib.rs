mod utils;
mod state;
mod config;
mod test_utils;
mod takoyaki;
mod errors;

use serde::Deserialize;
// Reexport 
pub use utils::*;
pub use takoyaki::*;
pub use test_utils::*;
pub use errors::*;
pub use config::*;
pub use state::*;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use super::*;
    use super::test_utils::test_utils::*;

    // #[test]
    // fn cache_path_does_not_exit() {
    //     let cache = Cache::new(PathBuf::from("SOME_RANDOM_SH*T"));
    //
    //     assert_eq!(cache.exists() , false);
    // }
    //
    // #[test]
    // fn cache_path_created_and_exists() {
    //     let cache = Cache::new(cache_dir());
    //
    //     assert!(cache.create().is_ok());
    //     assert!(cache.exists());
    // }
    //
    // #[test]
    // fn cache_folder_create_error() {
    //     let cache = Cache::new(PathBuf::from("/some/path/that/cannot/be/created"));
    //
    //     assert!(cache.create().is_err());
    //     assert_eq!(cache.exists() , false);
    // }
    //
    // #[tokio::test]
    // async fn no_ready_function() {
    //     let takoyaki = Takoyaki::new("new_plug");
    //     let response = takoyaki.start();
    //
    //     assert!(matches!(response.await.unwrap_err() , Errors::NoStartFunctionFound));
    // }
    //
    // #[tokio::test]
    // async fn no_execute_function() {
    //     let mut takoyaki = Takoyaki::new("new_plug");
    //
    //     takoyaki.set_ready(Box::new(|_ , _| { ReadyState::from_cache(Cache::new(cache_dir())) }));
    //
    //     let response = takoyaki.start();
    //
    //     assert!(matches!(response.await.unwrap_err() , Errors::NoExecuteFunctionFound));
    // }
    //
    // #[tokio::test]
    // async fn should_be_ok() {
    //     let mut takoyaki = Takoyaki::new("new_plug");
    //     takoyaki.set_ready(Box::new(|_ , _| { ReadyState::from_cache(Cache::new(cache_dir())) }));
    //     takoyaki.set_execute(Box::new(|| {  }));
    //
    //     let response = takoyaki.start();
    // }
    //
    #[tokio::test]
    async fn should_error_without_state() {
        let mut takoyaki = Takoyaki::new("new_plug");

        takoyaki.set_ready(Box::new(|cache , config| -> ReadyState { 
            if cache.exists() {
                return ReadyState::from_cache(cache)
            }

            let client = reqwest::Client::new();
            let mut body = HashMap::new();

            body.insert("query", r#"
                query {
                    user(login: "ThePrimeagen") {
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
                }
            "#);

            return ReadyState::from_reqwest(
                client
                    .post("https://api.github.com/graphql")
                    .body(serde_json::to_string(&body).unwrap())
                    .header("Authorization" , "Bearer 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")
            );
        }));

        takoyaki.set_execute(Box::new(|res: Root| -> PrintableGrid { 
            let mut printable = PrintableGrid::new();
            let mut x = 0;
            let mut y = 0;

            println!("Res");

            for week in res.data.user.contributions_collection.contribution_calendar.weeks {
                for day in week.contribution_days {
                    printable.insert(x , y, Printable { color: day.color , contribution_count: day.contribution_count });

                    x += 1;
                }

                x = 0;
                y += 1;
            }

            printable
        }));

        let response = takoyaki.start().await;

        // assert!(matches!(response.await.unwrap_err() , Errors::StateUnset));
    }
}

