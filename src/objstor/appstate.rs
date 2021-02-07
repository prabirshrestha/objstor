use super::UserBacked;

pub struct AppState<'a> {
    pub user: &'a dyn UserBacked,
}
