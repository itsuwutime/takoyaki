use reqwest::RequestBuilder;


pub struct ReadyState {
    response: Option<String>,
    pending: Option<RequestBuilder>
}

impl ReadyState {
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
        if let Some(client) = &self.pending {
            let resp = client.try_clone().unwrap().header("User-Agent", "takoyaki");

            resp.send().await?.json::<T>().await
        } else {
            Ok(T::default())
        }
    }
}
