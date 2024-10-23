use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    password: String,
    address: String,
}
