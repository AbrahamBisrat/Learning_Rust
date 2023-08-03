#![allow(dead_code, unused_variables)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,
    id: Option<u32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    // println!("Incoming Json data from Placeholder \n {:#?}", todos);

    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "some title".to_owned(),
        completed: false,
    };

    let new_todo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&serde_json::json!({
            "userId": 1,
            "title": "some title".to_owned(),
            "completed": false
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("Sent Json: {:#?}", &new_todo);

    Ok(())
}
