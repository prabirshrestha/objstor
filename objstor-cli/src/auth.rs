use crate::state::State;
use objstor::User;
use tide_http_auth::{BasicAuthRequest, Storage};

#[async_trait::async_trait]
impl Storage<User, BasicAuthRequest> for State {
    async fn get_user(&self, request: BasicAuthRequest) -> tide::Result<Option<User>> {
        if self
            .provider
            .validate_user(&request.username, &request.password)
            .await?
        {
            Ok(self
                .provider
                .get_user_by_username(&request.username)
                .await?)
        } else {
            Ok(None)
        }
    }
}
