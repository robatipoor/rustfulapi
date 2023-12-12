use tera::Tera;

use crate::dto::Template;

#[derive(Clone)]
pub struct TemplateEngine {
  tera: Tera,
}

impl TemplateEngine {
  pub fn new(path: &str) -> tera::Result<Self> {
    Ok(Self {
      tera: Tera::new(path)?,
    })
  }

  pub fn render(&self, template: &Template) -> Result<String, tera::Error> {
    let (ctx, path) = template.get();
    self.tera.render(path, &ctx)
  }
}

#[cfg(test)]
mod tests {
  use fake::{Fake, Faker};
  use uuid::Uuid;

  use crate::{constant::TEMPLATE_ENGIN, dto::Template};

  #[test]
  fn template_engin_test() {
    let username: String = Faker.fake();
    let code: String = Faker.fake();
    let user_id: Uuid = Faker.fake();
    let template = Template::Login2fa {
      username: username.clone(),
      user_id,
      code: code.clone(),
    };
    let content = TEMPLATE_ENGIN.render(&template).unwrap();
    assert!(content.contains(&username));
    assert!(content.contains(&code));
    assert!(content.contains(&user_id.to_string()));
  }
}
