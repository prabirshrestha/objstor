use async_trait::async_trait;
use tide::{http::Method, Middleware, Next, Request, Response, Result, StatusCode};

#[derive(Debug, Clone)]
pub struct WebdavMiddleware {}

impl WebdavMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for WebdavMiddleware {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> Result {
        match req.method() {
            Method::Options => self.handle_opts(req, next).await,
            _ => Ok(Response::new(StatusCode::BadRequest)),
        }
    }
}

impl WebdavMiddleware {
    async fn handle_opts<State: Clone + Send + Sync + 'static>(
        &self,
        req: Request<State>,
        next: Next<'_, State>,
    ) -> Result {
        let res = Response::builder(StatusCode::Ok)
            .header(
                "allow",
                "OPTIONS, LOCK, DELETE, PROPPATCH, COPY, MOVE, UNLOCK, PROPFIND",
                // if file: "OPTIONS, LOCK, GET, HEAD, POST, DELETE, PROPPATCH, COPY, MOVE, UNLOCK, PROPFIND, PUT",
            )
            .build();
        Ok(res)
    }
}
