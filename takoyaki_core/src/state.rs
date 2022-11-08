use reqwest::RequestBuilder;
use serde::Deserialize;
use crate::{Cache, Errors};

#[derive(Debug)]
pub enum Pending {
    Reqwest(RequestBuilder),
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
            state: Pending::Reqwest(builder)
        }
    }

    pub fn from_cache(cache: Cache) -> Self {
        Self {
            state: Pending::Cache(cache)
        }
    }

    pub fn set_reqwest(&mut self , builder: RequestBuilder) {
        self.state = Pending::Reqwest(builder)
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
                return Err(Errors::StateUnset)
            },
            Pending::Reqwest(builder) => {
                builder
                    .try_clone()
                    .unwrap()
                    .send()
                    .await
                    .map_err(|e| Errors::ReqwestError(e))?
                    .json::<T>()
                    .await
                    .map_err(|e| Errors::ReqwestError(e))
            },
            Pending::Cache(cache) => {
                return cache
                    .retrieve()
            }
        }
    }
}

