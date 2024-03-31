use mongodb::{Client, Collection};

use crate::models::user_model::{User, new_user};

pub async fn init_users(mongo_client: Client) {
    let users = vec![
        new_user("Debajyoti Saha".to_string(), "debu@email.com".to_string(), "admin".to_string()),
        new_user("Rajdeep Sengupta".to_string(), "raj@email.com".to_string(), "admin".to_string()),
        new_user("Sourav Banik".to_string(), "pachu@email.com".to_string(), "user".to_string()),
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
