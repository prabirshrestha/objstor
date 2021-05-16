use std::sync::Arc;

use objstor::ObjstorProvider;

#[derive(Clone)]
pub struct State {
    pub(crate) provider: Arc<Box<dyn ObjstorProvider>>,
}

impl State {
    pub fn new(provider: Box<dyn ObjstorProvider>) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }
}
