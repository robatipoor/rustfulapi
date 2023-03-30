use reqwest::{Client, Response};
use serde::Serialize;

pub async fn post<T: Serialize + ?Sized>(
  client: &Client,
  url: &str,
  body: &T,
) -> Result<Response, reqwest::Error> {
  client.post(url).json(body).send().await
}

pub async fn put<T: Serialize + ?Sized>(
  client: &Client,
  url: &str,
  body: &T,
) -> Result<Response, reqwest::Error> {
  client.put(url).json(body).send().await
}

pub async fn delete(client: &Client, url: &str) -> Result<Response, reqwest::Error> {
  client.delete(url).send().await
}

pub async fn get(client: &Client, url: &str) -> Result<Response, reqwest::Error> {
  client.get(url).send().await
}
