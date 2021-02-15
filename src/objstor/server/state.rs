use std::sync::Arc;

use crate::objstor::backend::ObjstorBackend;

#[derive(Clone)]
pub struct State {
    pub objstor_backend: Arc<dyn ObjstorBackend + Send + Sync + 'static>,
}

impl State {
    pub fn new(objstor_backend: Arc<dyn ObjstorBackend + Send + Sync + 'static>) -> Self {
        Self { objstor_backend }
    }
}
