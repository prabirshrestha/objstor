use crate::{api, config::Serve, state::State};
use anyhow::Result;
use objstor_sqlite_provider::SqliteObjstorProvider;
use rust_embed::RustEmbed;
use tide::{prelude::*, Request, Response, Server};
use webdav::WebdavHandler;

#[cfg(debug_assertions)]
#[derive(RustEmbed)]
#[folder = "../client/public/"]
#[prefix = "/"]
struct ClientAssets;

#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "../client/build/"]
#[prefix = "/"]
struct ClientAssets;

pub async fn serve(s: &Serve) -> Result<()> {
    // tide::log::start();

    let app = server(&s).await?;

    let state = app.state();
    state.provider.init().await?;

    let mut listener = app.bind((&s.host, s.port)).await?;
    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }

    listener.accept().await?;

    Ok(())
}

async fn server(s: &Serve) -> Result<Server<State>> {
    let mut app = tide::with_state(get_state(&s).await?);
    let state = app.state().clone();

    app.with(driftwood::ApacheCombinedLogger);

    app.at("/api").nest({
        let mut app = tide::with_state(state);
        app.at("/password").post(api::user::change_password);

        app.with(tide_http_auth::Authentication::new(
            tide_http_auth::BasicAuthScheme::default(),
        ));
        app.at("/users").post(api::user::create_user);
        app
    });

    // NOTE: due to bug in tide make sure to register multiple webdav routes.
    // https://github.com/http-rs/tide/issues/205
    app.at("/webdav").all(handle_webdav);
    app.at("/webdav/").all(handle_webdav);
    app.at("/webdav/*path").all(handle_webdav);

    app.at("/").all(handle_all);
    app.at("*").all(handle_all);

    Ok(app)
}

async fn get_state(s: &Serve) -> Result<State> {
    let provider = SqliteObjstorProvider::connect(&s.connection_string, &s.salt).await?;
    Ok(State::new(Box::new(provider)))
}

async fn handle_webdav(req: Request<State>) -> tide::Result {
    let webdav = WebdavHandler::new();
    Ok(webdav.handle(&req).await?)
}

#[cfg(debug_assertions)]
async fn handle_all(req: Request<State>) -> tide::Result {
    use tide::{http::Method, StatusCode};

    let url = "http://localhost:3000";

    if req.method() != Method::Get {
        return Ok(Response::builder(StatusCode::NotFound).build());
    }

    let path = if req.url().path() == "/" {
        "/index.html"
    } else {
        req.url().path()
    };

    let mut req_builder = surf::get(format!("{}{}", url, path));
    for (n, v) in req.iter().filter(|(n, _)| *n != "host") {
        let v: String = v.iter().map(|s| s.as_str()).collect();
        req_builder = req_builder.header(n, v);
    }
    let mut proxy_res = req_builder.send().await?;
    let mut res = Response::new(proxy_res.status());
    proxy_res.iter().for_each(|(n, v)| {
        res.append_header(n, v);
    });
    if let Some(mime) = proxy_res.content_type() {
        res.set_content_type(mime);
    }
    res.set_body(proxy_res.take_body());
    Ok(res)
}

#[cfg(not(debug_assertions))]
async fn handle_all(req: Request<State>) -> tide::Result {
    use std::str::FromStr;
    use tide::{
        http::{Method, Mime},
        Body, StatusCode,
    };

    if req.method() != Method::Get {
        return Ok(Response::builder(StatusCode::NotFound).build());
    }

    let path = &req.url().path();
    let res = match ClientAssets::get(path) {
        Some(content) => {
            let body: tide::Body = match content {
                std::borrow::Cow::Borrowed(bytes) => bytes.into(),
                std::borrow::Cow::Owned(bytes) => bytes.into(),
            };
            Response::builder(StatusCode::Ok)
                .content_type(Mime::from_str(
                    mime_guess::from_path(&path)
                        .first_or_octet_stream()
                        .as_ref(),
                )?)
                .body(body)
                .build()
        }
        _ => {
            let index_html = ClientAssets::get("/index.html").unwrap();
            Response::builder(StatusCode::Ok)
                .content_type(Mime::from_str(
                    mime_guess::from_path("/index.html")
                        .first_or_octet_stream()
                        .as_ref(),
                )?)
                .body(Body::from(std::str::from_utf8(&index_html)?))
                .build()
        }
    };
    Ok(res)
}
