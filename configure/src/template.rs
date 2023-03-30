use once_cell::sync::Lazy;
use tera::Tera;

use model::Template;

pub static TEMPLATE_ENGIN: Lazy<TemplateEngine> = Lazy::new(|| {
  let path = util::file::root_dir("static/template/**/*")
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap();
  TemplateEngine::new(&path).unwrap()
});

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

  use model::Template;

  use super::*;

  #[test]
  fn template_engin_test() {
    let username: String = Faker.fake();
    let code: String = Faker.fake();
    let template = Template::Login {
      username: username.clone(),
      code: code.clone(),
    };
    let content = TEMPLATE_ENGIN.render(&template).unwrap();
    assert!(content.contains(&*username));
    assert!(content.contains(&*code));
  }
}
