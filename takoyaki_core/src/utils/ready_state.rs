use serde::Deserialize;

use crate::{Cache, Result, Error};

pub enum Pending {
    Reqwest(Box<reqwest::RequestBuilder>),
    Cache(Cache),
    Unset
}

pub struct ReadyState {
    pending: Pending
}

impl ReadyState {
    pub fn new() -> Self {
        Self {
            pending: Pending::Unset
        }
    }

    pub fn from_cache(cache: Cache) -> Self {
        Self {
            pending: Pending::Cache(cache)
        }
    }

    pub fn from_reqwest(client: reqwest::RequestBuilder) -> Self {
        Self {
            pending: Pending::Reqwest(Box::new(client))
        }
    }

    pub async fn resolve<T>(&self) -> Result<T> 
    where 
        T: for<'de> Deserialize<'de>
    {
        match &self.pending {
            Pending::Reqwest(client) => {
                client
                    .try_clone()
                    .ok_or(Error::BuilderCloneError)?
                    .header("User-Agent" , "takoyaki")
                    .send()
                    .await
                    .map_err(|_| Error::ReqwestError)?
                    .json()
                    .await
                    .map_err(|_| Error::SerializationError)
            },
            Pending::Cache(cache) => {
                cache.retrieve::<T>()
            },
            Pending::Unset => {
                Err(Error::StateIsUnset)
            }
        }
    }
}

impl Default for ReadyState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn ready_state_without_pending() {
        let state = ReadyState::new();

        assert_eq!(state.resolve::<serde_json::Value>().await.unwrap_err() , Error::StateIsUnset);
    }
}
