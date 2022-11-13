use serde::{Deserialize , Serialize};

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

    pub async fn resolve<T>(&self , cache: Cache) -> Result<T> 
    where 
        T: for<'de> Deserialize<'de> + Serialize
    {
        match &self.pending {
            Pending::Reqwest(client) => {
                let resp = client
                    .try_clone()
                    .ok_or(Error::BuilderCloneError)?
                    .header("User-Agent" , "takoyaki")
                    .send()
                    .await
                    .map_err(|_| Error::ReqwestError)?
                    .json()
                    .await
                    .map_err(|_| Error::SerializationError)?;

                cache.write_as_str(&serde_json::to_string(&resp).unwrap())?;

                Ok(resp)
            },
            Pending::Cache(cache) => {
                if !cache.exists() {
                    Err(Error::CacheDoesNotExist)
                } else {
                    cache.retrieve::<T>()
                }
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

        assert_eq!(state.resolve::<serde_json::Value>(Cache::new("anyplug").unwrap()).await.unwrap_err() , Error::StateIsUnset);
    }
}
