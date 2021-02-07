use super::UserBackend;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub userbackend: Arc<Box<dyn UserBackend>>,
}
