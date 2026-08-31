#![forbid(unsafe_code, clippy::unwrap_used)]

use rust_embed::RustEmbed;
use trailbase_wasm::http::{header, HttpError, HttpRoute, IntoBody, Request, Response, StatusCode, routing};
use trailbase_wasm::{Guest, Metadata, export};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AssetRoute {
  Index,
  Asset,
  NotFound,
}

fn asset_route(path: &str, asset_exists: bool) -> AssetRoute {
  if path.is_empty() || path == "index.html" {
    return AssetRoute::Index;
  }
  if !path.contains('.') {
    return if asset_exists { AssetRoute::Asset } else { AssetRoute::Index };
  }
  if asset_exists {
    return AssetRoute::Asset;
  }
  return AssetRoute::NotFound;
}

struct Endpoints;

impl Guest for Endpoints {
  fn http_handlers() -> Vec<HttpRoute> {
    vec![
      routing::get("/_/admin-refresh/", root_handler),
      routing::get("/_/admin-refresh/{*wildcard}", async |req: Request| {
        static_handler(req.path_param("wildcard").unwrap_or_default()).await
      }),
    ]
  }

  fn metadata() -> Option<Metadata> {
    Some(Metadata {
      display_name: Some("TrailBase Admin UI Refresh".to_string()),
      description: Some("A refreshed TrailBase administration interface.".to_string()),
      admin_ui_path: Some("/_/admin-refresh/".to_string()),
      ..Default::default()
    })
  }
}

export!(Endpoints);

async fn root_handler(_: Request) -> Result<Response, HttpError> {
  serve_index()
}

async fn static_handler(path: &str) -> Result<Response, HttpError> {
  let file = Assets::get(path);
  match asset_route(path, file.is_some()) {
    AssetRoute::Index => serve_index(),
    AssetRoute::Asset => {
      let file = file.ok_or_else(|| HttpError::status(StatusCode::NOT_FOUND))?;
      let mut response = Response::builder().header(header::CONTENT_TYPE, file.metadata.mimetype());
      if let Some(cache) = cache_control(AssetRoute::Asset) {
        response = response.header(header::CACHE_CONTROL, cache);
      }
      response
        .body(file.data.into_body())
        .map_err(|_| HttpError::status(StatusCode::INTERNAL_SERVER_ERROR))
    }
    AssetRoute::NotFound => Err(HttpError::status(StatusCode::NOT_FOUND)),
  }
}

fn cache_control(route: AssetRoute) -> Option<&'static str> {
  (route == AssetRoute::Asset).then_some("public, max-age=604800, immutable")
}

fn serve_index() -> Result<Response, HttpError> {
  let file = Assets::get("index.html")
    .ok_or_else(|| HttpError::status(StatusCode::INTERNAL_SERVER_ERROR))?;
  Response::builder()
    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
    .body(file.data.into_body())
    .map_err(|_| HttpError::status(StatusCode::INTERNAL_SERVER_ERROR))
}

#[cfg(test)]
mod tests {
  use super::{AssetRoute, asset_route};

  #[test]
  fn routes_root_to_index() {
    assert_eq!(asset_route("", false), AssetRoute::Index);
  }

  #[test]
  fn serves_direct_assets() {
    assert_eq!(asset_route("assets/app.js", true), AssetRoute::Asset);
  }

  #[test]
  fn extensionless_paths_use_spa_fallback() {
    assert_eq!(asset_route("settings/users", false), AssetRoute::Index);
  }

  #[test]
  fn serves_known_extensionless_embedded_assets() {
    assert_eq!(asset_route("favicon", true), AssetRoute::Asset);
  }

  #[test]
  fn direct_index_is_not_immutable() {
    assert_eq!(asset_route("index.html", true), AssetRoute::Index);
    assert_eq!(super::cache_control(AssetRoute::Index), None);
    assert_eq!(super::cache_control(AssetRoute::Asset), Some("public, max-age=604800, immutable"));
  }

  #[test]
  fn missing_file_like_paths_are_not_found() {
    assert_eq!(asset_route("assets/missing.js", false), AssetRoute::NotFound);
  }
}
