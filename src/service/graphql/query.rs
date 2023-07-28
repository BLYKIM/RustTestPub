use async_graphql::{Object, Result};

#[derive(Default)]
pub(super) struct TempQuery;

#[Object]
impl TempQuery {
    async fn sample<'ctx>(&self) -> Result<String> {
        Ok(String::from("Success"))
    }
}
