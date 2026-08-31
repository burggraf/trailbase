#![forbid(unsafe_code, clippy::unwrap_used)]

use rust_embed::RustEmbed;
use trailbase_wasm::http::{HttpError, HttpRoute, IntoBody, IntoResponse, Request, Response, StatusCode, header, routing};
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
  if path.is_empty() || !path.contains('.') {
    return AssetRoute::Index;
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
      Response::builder()
        .header(header::CONTENT_TYPE, file.metadata.mimetype())
        .header(header::CACHE_CONTROL, "public, max-age=604800, immutable")
        .body(file.data.into_body())
        .map_err(|_| HttpError::status(StatusCode::INTERNAL_SERVER_ERROR))
    }
    AssetRoute::NotFound => Err(HttpError::status(StatusCode::NOT_FOUND)),
  }
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
  fn missing_file_like_paths_are_not_found() {
    assert_eq!(asset_route("assets/missing.js", false), AssetRoute::NotFound);
  }
}
