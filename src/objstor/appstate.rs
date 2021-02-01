use super::UserBacked;
use std::sync::Arc;

pub struct AppState<'a> {
    pub user: &'a UserBacked,
}
