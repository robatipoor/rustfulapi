use std::fmt::Display;

use super::http::CLIENT;
use anyhow::anyhow;
use configure::email::EmailConfig;
use reqwest::Client;
use reqwest::StatusCode;
use scraper::Html;
use scraper::Selector;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use tracing::info;
use util::http;

pub struct MailHogClient {
  client: &'static Client,
  addr: String,
}

impl MailHogClient {
  pub fn new(config: &EmailConfig) -> Self {
    Self {
      client: &*CLIENT,
      addr: config.host.clone(),
    }
  }
}

pub enum QueryKindSearch {
  From,
  To,
  Containing,
}

impl Display for QueryKindSearch {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Self::From => write!(f, "from"),
      Self::To => write!(f, "to"),
      Self::Containing => write!(f, "containing"),
    }
  }
}

pub struct MailHogResponse {}

impl MailHogClient {
  pub async fn get(&self, msg_id: String) -> Result<Item, reqwest::Error> {
    let resp = http::get(
      self.client,
      &format!("http://{}:8025/api/v1/messages/{msg_id}", self.addr),
    )
    .await?;
    let resp: Item = resp.json().await?;
    info!("get list mailhog {:?}", resp);
    Ok(resp)
  }

  pub async fn delete(&self, msg_id: String) -> Result<StatusCode, reqwest::Error> {
    let resp = http::delete(
      self.client,
      &format!("http://{}:8025/api/v1/messages/{msg_id}", self.addr),
    )
    .await?;
    Ok(resp.status())
  }

  pub async fn search(
    &self,
    query_kind: QueryKindSearch,
    query: &str,
  ) -> Result<Response, reqwest::Error> {
    let resp = http::get(
      self.client,
      &format!(
        "http://{}:8025/api/v2/search?kind={}&query={}",
        self.addr, query_kind, query
      ),
    )
    .await?;
    let resp: Response = resp.json().await?;
    info!("search mailhog {:?}", resp);
    Ok(resp)
  }

  pub async fn get_code_from_email(&self, email: &str) -> anyhow::Result<String> {
    let resp = self.search(QueryKindSearch::To, email).await?;
    let body = resp
      .items
      .get(0)
      .ok_or_else(|| anyhow!("item not found"))?
      .content
      .body
      .clone();
    let html = Html::parse_document(&body);
    let selector =
      Selector::parse(r#"strong"#).map_err(|e| anyhow!("parse strong tag failed {:?}", e))?;
    let token = html
      .select(&selector)
      .nth(1)
      .ok_or_else(|| anyhow!("item not found"))?
      .text()
      .collect::<String>();
    Ok(token)
  }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
  pub total: i64,
  pub count: i64,
  pub start: i64,
  pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  #[serde(rename = "ID")]
  pub id: String,
  #[serde(rename = "From")]
  pub from: From,
  #[serde(rename = "To")]
  pub to: Vec<To>,
  #[serde(rename = "Content")]
  pub content: Content,
  #[serde(rename = "Created")]
  pub created: String,
  #[serde(rename = "MIME")]
  pub mime: Value,
  #[serde(rename = "Raw")]
  pub raw: Raw,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From {
  #[serde(rename = "Relays")]
  pub relays: Value,
  #[serde(rename = "Mailbox")]
  pub mailbox: String,
  #[serde(rename = "Domain")]
  pub domain: String,
  #[serde(rename = "Params")]
  pub params: String,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct To {
  #[serde(rename = "Relays")]
  pub relays: Value,
  #[serde(rename = "Mailbox")]
  pub mailbox: String,
  #[serde(rename = "Domain")]
  pub domain: String,
  #[serde(rename = "Params")]
  pub params: String,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
  #[serde(rename = "Headers")]
  pub headers: Headers,
  #[serde(rename = "Body")]
  pub body: String,
  #[serde(rename = "Size")]
  pub size: i64,
  #[serde(rename = "MIME")]
  pub mime: Value,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
  #[serde(rename = "Content-Transfer-Encoding")]
  pub content_transfer_encoding: Vec<String>,
  #[serde(rename = "Date")]
  pub date: Vec<String>,
  #[serde(rename = "From")]
  pub from: Vec<String>,
  #[serde(rename = "Message-ID")]
  pub message_id: Vec<String>,
  #[serde(rename = "Received")]
  pub received: Vec<String>,
  #[serde(rename = "Return-Path")]
  pub return_path: Vec<String>,
  #[serde(rename = "Subject")]
  pub subject: Vec<String>,
  #[serde(rename = "To")]
  pub to: Vec<String>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Raw {
  #[serde(rename = "From")]
  pub from: String,
  #[serde(rename = "To")]
  pub to: Vec<String>,
  #[serde(rename = "Data")]
  pub data: String,
}

#[cfg(test)]
mod tests {

  use fake::{Fake, Faker};

  use client::email::{EmailClient, EmailClientExt};
  use configure::{template::TEMPLATE_ENGIN, CONFIG};
  use model::{Email, Template};

  use super::*;

  #[tokio::test]
  async fn test_mailhog_search() {
    let email: Email = Faker.fake();
    let email_client = EmailClient::new(&CONFIG.email).await.unwrap();
    email_client.send_email(&email).await.unwrap();
    let mailer = MailHogClient::new(&CONFIG.email);
    let mut resp = mailer.search(QueryKindSearch::To, &email.to).await.unwrap();
    for _ in 0..10 {
      if resp.total > 0 {
        break;
      }
      tokio::time::sleep(std::time::Duration::from_nanos(1000)).await;
      resp = mailer.search(QueryKindSearch::To, &email.to).await.unwrap();
    }
    assert!(resp.total > 0);
  }

  #[tokio::test]
  async fn test_mailhog_get_token() {
    let code: String = Faker.fake();
    let username: String = Faker.fake();
    let template = Template::Invitation {
      username,
      code: code.clone(),
    };
    let body = TEMPLATE_ENGIN.render(&template).unwrap();
    let mut email: Email = Faker.fake();
    email.body = body;
    let email_client = EmailClient::new(&CONFIG.email).await.unwrap();
    email_client.send_email(&email).await.unwrap();
    let mailer = MailHogClient::new(&CONFIG.email);
    let mut resp: Option<String> = None;
    for _ in 0..10 {
      if let Ok(r) = mailer.get_code_from_email(&email.to).await {
        resp = Some(r);
        break;
      }
      tokio::time::sleep(std::time::Duration::from_nanos(1000)).await;
    }
    assert!(matches!(resp, Some(c) if c == code));
  }

  #[tokio::test]
  async fn test_mailhog_get() {
    let email: Email = Faker.fake();
    let email_client = EmailClient::new(&CONFIG.email).await.unwrap();
    email_client.send_email(&email).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let mailer = MailHogClient::new(&CONFIG.email);
    let mut resp = mailer.search(QueryKindSearch::To, &email.to).await.unwrap();
    for _ in 0..10 {
      if resp.total > 0 {
        break;
      }
      tokio::time::sleep(std::time::Duration::from_nanos(1000)).await;
      resp = mailer.search(QueryKindSearch::To, &email.to).await.unwrap();
    }
    assert!(resp.total > 0);
    mailer.get(resp.items[0].id.clone()).await.unwrap();
  }

  #[tokio::test]
  async fn test_mailhog_delete() {
    let email: Email = Faker.fake();
    let email_client = EmailClient::new(&CONFIG.email).await.unwrap();
    email_client.send_email(&email).await.unwrap();
    let mailer = MailHogClient::new(&CONFIG.email);
    let mut resp = mailer.search(QueryKindSearch::To, &email.to).await.unwrap();
    for _ in 0..10 {
      if resp.total > 0 {
        break;
      }
      tokio::time::sleep(std::time::Duration::from_nanos(1000)).await;
      resp = mailer.search(QueryKindSearch::To, &email.to).await.unwrap();
    }
    assert!(resp.total > 0);
    mailer.delete(resp.items[0].id.clone()).await.unwrap();
  }
}
