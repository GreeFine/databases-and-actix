use mongodb::bson::{doc, to_bson};
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Library {
    nane: String,
    books: Vec<Book>,
    alphabetical_book_list: HashMap<char, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
    borowers: Option<Vec<User>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    name: String,
    age: u8,
}

#[actix_rt::test]
async fn mongo() {
    let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017")
        .await
        .expect("Unable to connect to the database");
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("tests");

    let typed_collection = db.collection::<Library>("libraries");
    let librarie_name = "Nia Gutmann";
    let filter = doc! { "nane": librarie_name };
    let user = User {
        name: "Raphael".to_string(),
        age: 18,
    };
    let new_books = vec![
        Book {
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik".to_string(),
            borowers: None,
        },
        Book {
            title: "Crust of Rust".to_string(),
            author: "on Gjengset".to_string(),
            borowers: Some(vec![user.clone()]),
        },
    ];
    let new_books_serialized = to_bson(&new_books).expect("Unable to convert orders to bson");
    let update_result = typed_collection
        .update_one(
            filter,
            doc! {
              "$set": { "books": new_books_serialized },
              "$push": {
                "alphabetical_book_list.c": "Crust of Rust",
                "alphabetical_book_list.t": "The Rust Programming Language",
              },
            },
            None,
        )
        .await
        .unwrap();
    if update_result.matched_count == 0 {
        eprintln!("Didn't find the librarie {}", librarie_name);
    } else if update_result.modified_count == 0 {
        eprintln!("Didn't update the library books");
    };
}
