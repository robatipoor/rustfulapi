use actix_cors::Cors;
use configure::Profile;

pub fn get_cors_config(domain: String, profile: Profile) -> Cors {
  Cors::default()
    .send_wildcard()
    .allowed_origin_fn(move |origin, _req_head| {
      if let Ok(origin) = origin.to_str() {
        if origin.ends_with(&domain) {
          return true;
        }
      } else {
        return false;
      }
      if !matches!(profile, Profile::Prod) {
        return true;
      }
      false
    })
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
    .allow_any_header()
    .supports_credentials()
    .max_age(3600)
}
