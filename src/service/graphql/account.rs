use super::Database;
use async_graphql::{Context, Object, Result, SimpleObject};
use chrono::NaiveDateTime;

#[derive(Default)]
pub(super) struct AccountMutation;

#[Object]
impl AccountMutation {
    async fn sign_in<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
        password: String,
    ) -> Result<User> {
        let db = ctx.data::<Database>()?;

        if db.check_account(&username, &password).await? {
            Ok(User {
                username,
                token: "token".to_string(),
                expiration_time: NaiveDateTime::MAX,
            })
        } else {
            Err("incorrect username or password".into())
        }
    }
}

#[derive(SimpleObject)]
struct User {
    username: String,
    token: String,
    expiration_time: NaiveDateTime,
}
