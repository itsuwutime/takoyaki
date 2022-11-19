use anyhow::Result;
use crate::LOGGER;
use colored::*;
use uuid::Uuid;
use async_trait::async_trait;

use crate::{Command , create_new_deployment};

pub struct Deploy {

}

#[async_trait]
impl Command for Deploy {
    fn new() -> Self {
        Self {

        }
    }

    fn prefix(&self) -> &str {
        "/deploy"
    }

    async fn respond(&self , args: Vec<&str>) -> Result<String> {
        if args.len() < 3 {
            return Ok("Must provide atleast 3 arguments in format: /deploy <git_repo_url> <branch> <path>".to_string())
        }

        // Create a new uuid
        let uuid = Uuid::new_v4().to_string();

        // Log 
        LOGGER.render(format!("Create a new build with the uuid of {}" , uuid).magenta().bold());

        // Create a new build
        create_new_deployment(&uuid , args[0] , args[1] , args[2] , args[3]);

        Ok(uuid)
    }
}

