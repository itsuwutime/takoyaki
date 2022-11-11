use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use crate::{Cache, Error};

#[derive(Debug)]
pub enum Pending<'a> {
    Reqwest(Box<RequestBuilder>),
    Cache(&'a Cache),
    Unset
}

#[derive(Debug)]
pub struct ReadyState<'a> {
    state: Pending<'a>
}

impl<'a> ReadyState<'a> {
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

    pub fn from_cache(cache: &'a Cache) -> Self {
        Self {
            state: Pending::Cache(cache)
        }
    }

    pub fn set_reqwest(&mut self , builder: RequestBuilder) {
        self.state = Pending::Reqwest(Box::new(builder))
    }

    pub fn set_cache(&mut self , cache: &'a Cache) {
        self.state = Pending::Cache(cache)
    }

    pub async fn resolve<T>(&self , cache: &'a Cache) -> Result<T , Error> 
    where
        T: for<'de> Deserialize<'de> + Serialize
    {
        match &self.state {
            Pending::Unset => {
                Err(Error::StateUnset)
            },
            Pending::Reqwest(builder) => {
                let resp = builder
                    .try_clone()
                    .unwrap()
                    .header("User-Agent", "takoyaki")
                    .send()
                    .await
                    .map_err(Error::ReqwestError)?
                    .json::<T>()
                    .await
                    .map_err(Error::ReqwestError);

                cache.populate_as_str(&serde_json::to_string(&resp).unwrap());

                resp
            },
            Pending::Cache(cache) => {
                cache
                    .retrieve()
            }
        }
    }
}

