use std::fmt::Display;
use std::str::FromStr;

use anyhow::anyhow;
use reqwest::StatusCode;
use rustfulapi::client::http::HttpClientExt;
use rustfulapi::configure::email::EmailConfig;
use rustfulapi::constant::HTTP;
use scraper::Html;
use scraper::Selector;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use tracing::info;
use uuid::Uuid;

pub struct MailHogClient {
  addr: String,
}

impl MailHogClient {
  pub fn new(config: &EmailConfig) -> Self {
    Self {
      addr: config.host.clone(),
    }
  }
}

#[allow(dead_code)]
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

impl MailHogClient {
  pub async fn get(&self, msg_id: String) -> Result<Item, reqwest::Error> {
    let resp = HTTP
      .get_request(&format!(
        "http://{}:8025/api/v1/messages/{msg_id}",
        self.addr
      ))
      .await?;
    let resp: Item = resp.json().await?;
    info!("get list mailhog {:?}", resp);
    Ok(resp)
  }

  pub async fn delete(&self, msg_id: String) -> Result<StatusCode, reqwest::Error> {
    let resp = HTTP
      .delete_request(&format!(
        "http://{}:8025/api/v1/messages/{msg_id}",
        self.addr
      ))
      .await?;
    Ok(resp.status())
  }

  pub async fn search(
    &self,
    query_kind: QueryKindSearch,
    query: &str,
  ) -> Result<Response, reqwest::Error> {
    let resp = || async {
      let resp = HTTP
        .get_request(&format!(
          "http://{}:8025/api/v2/search?kind={}&query={}",
          self.addr, query_kind, query
        ))
        .await?;
      let resp: Response = resp.json().await?;
      Ok(resp)
    };
    let resp = rustfulapi::retry!(resp, |r: &Result<Response, reqwest::Error>| {
      match r {
        Ok(r) => !r.items.is_empty(),
        Err(_) => false,
      }
    });
    info!("Search mailhog {resp:?}.");
    resp
  }

  pub async fn get_code_and_id_from_email(&self, email: &str) -> anyhow::Result<(String, Uuid)> {
    let resp = self.search(QueryKindSearch::To, email).await?;
    let body = resp
      .items
      .first()
      .ok_or_else(|| anyhow!("Item not found"))?
      .content
      .body
      .clone();
    let html = Html::parse_document(&body);
    let selector =
      Selector::parse(r#"strong"#).map_err(|e| anyhow!("parse strong tag failed {:?}", e))?;
    let user_id = Uuid::from_str(
      &html
        .select(&selector)
        .nth(1)
        .ok_or_else(|| anyhow!("Item not found"))?
        .text()
        .collect::<String>(),
    )?;
    let code = html
      .select(&selector)
      .nth(2)
      .ok_or_else(|| anyhow!("Item not found"))?
      .text()
      .collect::<String>();
    let _ = self.delete(resp.items.first().unwrap().id.clone()).await;
    Ok((code, user_id))
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

  use super::*;
  use fake::{Fake, Faker};
  use rustfulapi::{
    client::{
      email::{EmailClient, EmailClientExt},
      ClientBuilder,
    },
    constant::{CONFIG, TEMPLATE_ENGIN},
    dto::{Email, Template},
  };

  #[tokio::test]
  async fn test_success_mail_search() {
    let email: Email = Faker.fake();
    let email_client = EmailClient::build_from_config(&CONFIG).unwrap();
    email_client.send_email(&email).await.unwrap();
    let mailer = MailHogClient::new(&CONFIG.email);
    let resp = mailer.search(QueryKindSearch::To, &email.to).await.unwrap();
    assert!(resp.total > 0);
  }

  #[tokio::test]
  async fn test_get_token_and_id_from_email() {
    let code: String = Faker.fake();
    let username: String = Faker.fake();
    let user_id: uuid::Uuid = Faker.fake();
    let template = Template::ActiveUser {
      username,
      user_id,
      code: code.clone(),
    };
    let body = TEMPLATE_ENGIN.render(&template).unwrap();
    let mut email: Email = Faker.fake();
    email.body = body;
    let email_client = EmailClient::build_from_config(&CONFIG).unwrap();
    email_client.send_email(&email).await.unwrap();
    let mailer = MailHogClient::new(&CONFIG.email);
    let (result_code, result_user_id) = mailer.get_code_and_id_from_email(&email.to).await.unwrap();
    assert_eq!(result_code, code);
    assert_eq!(result_user_id, user_id);
  }

  #[tokio::test]
  async fn test_success_mail_get() {
    let email: Email = Faker.fake();
    let email_client = EmailClient::build_from_config(&CONFIG).unwrap();
    email_client.send_email(&email).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let mailer = MailHogClient::new(&CONFIG.email);
    let resp = mailer.search(QueryKindSearch::To, &email.to).await.unwrap();
    assert!(resp.total > 0);
    mailer.get(resp.items[0].id.clone()).await.unwrap();
  }

  #[tokio::test]
  async fn test_success_mail_delete() {
    let email: Email = Faker.fake();
    let email_client = EmailClient::build_from_config(&CONFIG).unwrap();
    email_client.send_email(&email).await.unwrap();
    let mailer = MailHogClient::new(&CONFIG.email);
    let resp = mailer.search(QueryKindSearch::To, &email.to).await.unwrap();
    assert!(resp.total > 0);
    let status = mailer.delete(resp.items[0].id.clone()).await.unwrap();
    assert_eq!(status, reqwest::StatusCode::OK)
  }
}
