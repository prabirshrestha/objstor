use std::borrow::Cow;

use anyhow::Result;
use crate::{config::Serve, state::State};
use rust_embed::RustEmbed;
use sqlite_provider::SqliteObjstorProvider;
use std::str::FromStr;
use tide::{Body, Request, Response, Server, StatusCode, http::Mime, prelude::*};
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
    let app = get_app(&s).await?;

    println!("Initializing server...");
    let state = app.state();
    state.provider.init().await?;
    println!("Initializing server complete...");

    let mut listener = app.bind((&s.host, s.port)).await?;
    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }

    listener.accept().await?;

    Ok(())
}

async fn get_app(s: &Serve) -> Result<Server<State>> {
    // tide::log::start();
    let state = get_state(&s).await?;
    let mut app = tide::with_state(state);

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
    let url = "http://localhost:3000";

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
    proxy_res.iter().filter(|(n, _)| *n != "content-encoding").for_each(|(n, v)| {
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
    let path = &req.url().path();
    let res = match ClientAssets::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into()
            };
            Response::builder(StatusCode::Ok)
                .content_type(Mime::from_str(mime_guess::from_path(&path).first_or_octet_stream().as_ref())?)
                .body(body)
                .build()
        },
        _ => {
            let index_html = ClientAssets::get("/index.html").unwrap();
            Response::builder(StatusCode::Ok)
                .content_type(Mime::from_str(mime_guess::from_path("/index.html").first_or_octet_stream().as_ref())?)
                .body(Body::from(std::str::from_utf8(&index_html)?))
                .build()
        }
    };
    Ok(res)
}
