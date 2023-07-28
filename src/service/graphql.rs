#![allow(clippy::unused_async)]

mod account;
mod query;

use crate::service::{Database, Store};
use async_graphql::{EmptySubscription, MergedObject};

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

#[derive(Default, MergedObject)]
pub struct Query(query::TempQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(account::AccountMutation);

#[must_use]
pub fn schema(database: Database, store: Store) -> Schema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(database)
        .data(store)
        .finish()
}
