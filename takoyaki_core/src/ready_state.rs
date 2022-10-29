// Import dependencies
use reqwest::RequestBuilder;
use serde::Serialize;

// Import inbuilt modules
use crate::Cache;

// Pendind state types - Either its gonna make request or use existing cache
pub enum Pending {
    Reqwest(Box<RequestBuilder>),
    Cache(Cache)
}

// Ready state
pub struct ReadyState {
    pending: Option<Pending>
}

// Add functions
impl ReadyState {
    // Use state from cache
    pub fn from_cache(cache: Cache) -> Self {
        Self {
            pending: Some(Pending::Cache(cache))
        }
    }

    // Use state from reqwest
    pub fn from_reqwest(client: RequestBuilder) -> Self {
        Self {
            pending: Some(Pending::Reqwest(Box::new(client)))
        }
    }
    
    // Resolve state - get the response as T
    pub async fn resolve<'a , T>(&mut self, cache: Cache) -> Result<T , reqwest::Error>
    where
        T: Default + for<'de> serde::Deserialize<'de> + Serialize
    {
        // Check pending state type
        match &self.pending {
            // Does pending state contain anything?
            Some(client) => {
                // Check the type of the Pending
                match client {
                    // Its a reqwest!
                    Pending::Reqwest(client) => {
                        // Make the request
                        let res = client.try_clone().expect("Error while creating a clone of the `RequestBuilder`")
                            .header("User-Agent" , "takoyaki") // Requests may need a User-Agent
                            .send()
                            .await?
                            .json::<T>()
                            .await?;

                        // Update the cache according to the new response
                        cache.populate(&res).expect("Error while writing to cache!");

                        // Return response
                        Ok(res)
                    },
                    // Its a cache!
                    Pending::Cache(cache) => {
                        // Return the cache as T
                        Ok(cache.get().unwrap())
                    }
                }

            },
            // No state found, maybe the dev forgot! They should be reminded
            None => {
                // PANIC!
                panic!("No state found!")
            }
        }
    }
}
