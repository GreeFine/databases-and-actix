#![feature(test)]
#![feature(explicit_generic_args_with_impl_trait)]
#![feature(async_closure)]
mod readmetest;

extern crate test;

use serde::{Deserialize, Serialize};
use test::Bencher;

use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};

#[allow(soft_unstable)]

pub async fn connect() -> mongodb::Collection<User> {
    let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017")
        .await
        .expect("Unable to connect to the database");
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("tests");

    db.collection::<User>("Users")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
}

pub async fn find_one_user(users: &Collection<User>, name: &str) -> Option<User> {
    users
        .find_one(doc! { "name": &name }, None)
        .await
        .expect("DB error: Unable to get User")
}

pub async fn insert_one_user(users: &Collection<User>, user: &User) {
    users
        .insert_one(user, None)
        .await
        .expect("DB error: Unable to insert User");
}

#[bench]
fn test_serde_insert(b: &mut Bencher) {
    use tokio::runtime::Runtime;

    // Create the runtime
    let rt = Runtime::new().unwrap();
    let handle = rt.handle();

    // Execute the future, blocking the current thread until completion
    let mut user_collection = None;
    handle.block_on(async {
        user_collection = Some(connect().await);
    });

    let user_collection = user_collection.unwrap();
    let new_user = User {
        name: "test".to_string(),
    };
    b.iter(|| {
        handle.block_on(async {
            insert_one_user(&user_collection, &new_user).await;
        });
    });
}

#[bench]
fn test_serde_find(b: &mut Bencher) {
    use tokio::runtime::Runtime;

    // Create the runtime
    let rt = Runtime::new().unwrap();
    let handle = rt.handle();

    // Execute the future, blocking the current thread until completion
    let mut user_collection = None;
    handle.block_on(async {
        user_collection = Some(connect().await);
    });

    let user_collection = user_collection.unwrap();
    b.iter(|| {
        handle.block_on(async {
            let res = find_one_user(&user_collection, "test").await;
            assert!(res.is_some());
        });
    });
}

fn main() {}
