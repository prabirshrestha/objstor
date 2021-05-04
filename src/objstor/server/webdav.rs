use async_trait::async_trait;
use async_vfs::Vfs;
use tide::{http::Method, Middleware, Next, Request, Response, Result, StatusCode};

pub struct WebdavMiddleware {
    vfs: Box<dyn Vfs + Send + Sync>,
}

impl WebdavMiddleware {
    pub fn new(vfs: Box<dyn Vfs + Send + Sync>) -> Self {
        Self { vfs }
    }
}

#[async_trait]
impl<State> Middleware<State> for WebdavMiddleware
where
    State: Clone + Send + Sync + 'static,
{
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        match req.method() {
            Method::Options => self.handle_options(req, next).await,
            _ => Ok(Response::new(StatusCode::BadRequest)),
        }
    }
}

impl WebdavMiddleware {
    async fn handle_options<State>(&self, _req: Request<State>, _next: Next<'_, State>) -> Result
    where
        State: Clone + Send + Sync + 'static,
    {
        // curl -X OPTIONS http://127.0.0.1:3000/webdav -vv
        let allow = if self.vfs.metadata("/").await?.is_dir() {
            "OPTIONS, LOCK, DELETE, PROPPATCH, COPY, MOVE, UNLOCK, PROPFIND"
        } else {
            "OPTIONS, LOCK, GET, HEAD, POST, DELETE, PROPPATCH, COPY, MOVE, UNLOCK, PROPFIND, PUT"
        };

        let res = Response::builder(StatusCode::Ok)
            .header("allow", allow)
            .header("dav", "1")
            .header("ms-author-dav", "dav")
            .build();

        Ok(res)
    }
}
