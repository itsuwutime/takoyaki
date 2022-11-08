use reqwest::RequestBuilder;
use serde::Deserialize;
use crate::{Cache, Errors};

#[derive(Debug)]
pub enum Pending {
    Reqwest(Box<RequestBuilder>),
    Cache(Cache),
    Unset
}

#[derive(Debug)]
pub struct ReadyState {
    state: Pending
}

impl ReadyState {
    pub fn empty() -> Self {
        Self {
            state: Pending::Unset
        }
    }

    pub fn from_reqwest(builder: RequestBuilder) -> Self {
        Self {
            state: Pending::Reqwest(Box::new(builder))
        }
    }

    pub fn from_cache(cache: Cache) -> Self {
        Self {
            state: Pending::Cache(cache)
        }
    }

    pub fn set_reqwest(&mut self , builder: RequestBuilder) {
        self.state = Pending::Reqwest(Box::new(builder))
    }

    pub fn set_cache(&mut self , cache: Cache) {
        self.state = Pending::Cache(cache)
    }

    pub async fn resolve<T>(&self) -> Result<T , Errors> 
    where
        T: for<'de> Deserialize<'de>
    {
        match &self.state {
            Pending::Unset => {
                Err(Errors::StateUnset)
            },
            Pending::Reqwest(builder) => {
                builder
                    .try_clone()
                    .unwrap()
                    .header("User-Agent", "takoyaki")
                    .send()
                    .await
                    .map_err(Errors::ReqwestError)?
                    .json::<T>()
                    .await
                    .map_err(Errors::ReqwestError)
            },
            Pending::Cache(cache) => {
                cache
                    .retrieve()
            }
        }
    }
}

