use tide::{Request, Response, StatusCode};

pub struct WebdavHandler {}

impl WebdavHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WebdavHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl WebdavHandler {
    pub async fn handle<TState>(&self, req: &Request<TState>) -> tide::Result
    where
        TState: Send + Sync + 'static,
    {
        handle_not_implemented(req)
    }
}

fn handle_not_implemented<TState>(req: &Request<TState>) -> tide::Result
where
    TState: Send + Sync + 'static,
{
    let path = get_path(&req);

    let res = Response::builder(StatusCode::NotImplemented)
        .body(format!("Not implemented: {} {}", req.method(), path))
        .build();

    Ok(res)
}

fn get_path<TState>(req: &Request<TState>) -> String
where
    TState: Send + Sync + 'static,
{
    format!("/{}", req.param("path").unwrap_or(""))
}
