use chrono::Utc;
use mongodb::{Client, Collection};

use crate::models::user_model::{NewUser, User};

pub async fn init_users(mongo_client: Client) {
    let users = vec![
        NewUser {
            name: "Debajyoti Saha".to_string(),
            email: "debu@email.com".to_string(),
            role: "admin".to_string(),
            created_at: Utc::now().to_string(),
        },
        NewUser {
            name: "Rajdeep Sengupta".to_string(),
            email: "raj@email.com".to_string(),
            role: "admin".to_string(),
            created_at: Utc::now().to_string(),
        },
        NewUser {
            name: "Sourav Banik".to_string(),
            email: "sourav@email.com".to_string(),
            role: "user".to_string(),
            created_at: Utc::now().to_string(),
        },
    ];
    // check if the user collection is empty
    let user_collection: Collection<User> = mongo_client.database("test").collection("users");
    let count = user_collection.count_documents(None, None).await.unwrap();
    if count == 0 {
        // batch insert users using insert_many
        let insert_many_result = mongo_client
            .database("test")
            .collection("users")
            .insert_many(users, None)
            .await
            .unwrap();

        println!(">> init_users called");
        println!(">> Inserted users: {:?}", insert_many_result.inserted_ids);
    } else {
        println!(">> Users already exist in the database");
    }
}
