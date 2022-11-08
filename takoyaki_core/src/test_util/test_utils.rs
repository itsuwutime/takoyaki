#[cfg(test)]
pub mod test_utils {
    use std::path::PathBuf;

    use serde::{Serialize , Deserialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Root {
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
        pub total_contributions: usize,
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
        pub contribution_count: usize,
        pub date: String,
        pub weekday: i64,
    }


    pub fn cache_dir() -> PathBuf {
        dirs::cache_dir().unwrap().join("takoyaki")
    } 
}

