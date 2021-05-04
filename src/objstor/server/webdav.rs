use std::sync::Arc;

use async_trait::async_trait;
use async_vfs::Vfs;
use tide::{http::Method, Middleware, Next, Request, Response, Result, StatusCode};

#[derive(Clone)]
pub struct WebdavMiddleware {
    vfs: Arc<Box<dyn Vfs + Send + Sync + 'static>>,
}

impl WebdavMiddleware {
    pub fn new(vfs: Box<dyn Vfs + Send + Sync + 'static>) -> Self {
        Self { vfs: Arc::new(vfs) }
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for WebdavMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        match req.method() {
            Method::Options => self.handle_opts(req, next).await,
            _ => Ok(Response::new(StatusCode::BadRequest)),
        }
    }
}

impl WebdavMiddleware {
    async fn handle_opts<State: Clone + Send + Sync + 'static>(
        &self,
        _req: Request<State>,
        _next: Next<'_, State>,
    ) -> Result {
        // curl -X OPTIONS http://127.0.0.1:3000/webdav -vv
        let res = Response::builder(StatusCode::Ok)
            .header(
                "allow",
                "OPTIONS, LOCK, DELETE, PROPPATCH, COPY, MOVE, UNLOCK, PROPFIND",
                // if file: "OPTIONS, LOCK, GET, HEAD, POST, DELETE, PROPPATCH, COPY, MOVE, UNLOCK, PROPFIND, PUT",
            )
            .header("dav", "1")
            .header("ms-author-dav", "dav")
            .build();
        Ok(res)
    }
}
