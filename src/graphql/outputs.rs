use async_graphql::{OutputType, SimpleObject};

#[derive(SimpleObject)]
#[graphql(concrete(name = "UserDeleteResult", params(String)))]
#[graphql(concrete(name = "PermissionDeleteResult", params(i32)))]
pub(crate) struct DeleteResult<T: OutputType> {
    pub last_removed: T,
}

#[derive(SimpleObject)]
pub(crate) struct RegenerateAccessTokenResult {
    pub token: String,
}

#[derive(SimpleObject)]
pub(crate) struct DownloadUrlResult {
    pub url: String,
}
