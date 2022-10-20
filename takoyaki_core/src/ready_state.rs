use reqwest::RequestBuilder;

pub struct ReadyState {
    response: Option<String>,
    pending: Option<RequestBuilder>
}

impl ReadyState {
    pub fn from_cache(cache: String) -> Self {
        Self {
            response: Some(cache),
            pending: None
        }
    }

    pub fn from_reqwest(client: RequestBuilder) -> Self {
        Self {
            response: None,
            pending: Some(client)
        }
    }

    pub async fn resolve<'a , T>(&mut self) -> Result<T , reqwest::Error>
    where
        T: Default + for<'de> serde::Deserialize<'de>
    {
        match &self.pending {
            Some(client) => {
                Ok(client.try_clone().expect("Error while creating a clone of the `RequestBuilder`")
                    .header("User-Agent" , "takoyaki") // Requests may need a User-Agent
                    .send()
                    .await?
                    .json::<T>()
                    .await?
                )
                // let resp = client.try_clone().expect("Error while creating a clone of the `RequestBuilder`")
                //     .header("User-Agent" , "takoyaki") // Requests may need a User-Agent
                //     .send()
                //     .await?
                //     .json::<T>()
                //     .await?
                // ;
                //
                // Ok(T::default())
            },
            None => {
                Ok(serde_json::from_str(self.response.as_ref().unwrap().as_ref()).unwrap())
            }
        }
    }
}
