use reqwest::RequestBuilder;
use serde::Serialize;

use crate::cache::Cache;

pub enum Pending {
    Reqwest(Box<RequestBuilder>),
    Cache(Cache)
}

pub struct ReadyState {
    response: Option<String>,
    pending: Option<Pending>
}

impl ReadyState {
    pub fn from_cache(cache: Cache) -> Self {
        Self {
            response: None,
            pending: Some(Pending::Cache(cache))
        }
    }

    pub fn from_reqwest(client: RequestBuilder) -> Self {
        Self {
            response: None,
            pending: Some(Pending::Reqwest(Box::new(client)))
        }
    }

    pub async fn resolve<'a , T>(&mut self, cache: Cache) -> Result<T , reqwest::Error>
    where
        T: Default + for<'de> serde::Deserialize<'de> + Serialize
    {
        match &self.pending {
            Some(client) => {
                match client {
                    Pending::Reqwest(client) => {
                        let res = client.try_clone().expect("Error while creating a clone of the `RequestBuilder`")
                            .header("User-Agent" , "takoyaki") // Requests may need a User-Agent
                            .send()
                            .await?
                            .json::<T>()
                            .await?;

                        cache.populate(&res).expect("Error while writing to cache!");

                        Ok(res)
                    },
                    Pending::Cache(cache) => {
                        Ok(cache.get().unwrap())
                    }
                }

            },
            None => {
                Ok(serde_json::from_str(self.response.as_ref().unwrap().as_ref()).unwrap())
            }
        }
    }
}
