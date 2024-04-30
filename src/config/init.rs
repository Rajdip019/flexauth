use mongodb::{Client, Collection};
use std::env;

use crate::{
    core::user::User, models::dek_model::Dek, utils::encryption_utils::encrypt_data,
};

struct InitUser {
    name: String,
    email: String,
    role: String,
    password: String,
}

pub async fn init_users(mongo_client: Client) {
    // create a few users
    let users = vec![
        InitUser {
            name: "Debajyoti Saha".to_string(),
            email: "debajyotisaha14@gmail.com".to_string(),
            role: "admin".to_string(),
            password: "debu14@".to_string(),
        },
        InitUser {
            name: "Rajdeep Sengupta".to_string(),
            email: "rajdipgupta019@gmail.com".to_string(),
            role: "admin".to_string(),
            password: "raj19@".to_string(),
        },
        InitUser {
            name: "Sourav Banik".to_string(),
            email: "pachu@email.com".to_string(),
            role: "user".to_string(),
            password: "pachu20@".to_string(),
        },
    ];

    // check if the users already exist
    let db = mongo_client.database("test");
    let collection: Collection<User> = db.collection("users");
    let cursor = collection.count_documents(None, None).await.unwrap();

    if cursor > 0 {
        println!(">> Users already exist. Skipping user creation.");
        return;
    }

    // map the users
    for user in users {
        // create a new user
        let new_user = User::new_user(&user.name, &user.email, &user.role, &user.password);
        let dek = match new_user.encrypt_and_add(&mongo_client).await {
            Ok(dek) => dek,
            Err(e) => {
                println!(">> Error adding user: {:?}", e);
                continue;
            }
        };

        // encrypt the data with kek
        let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");
        let encrypted_dek = encrypt_data(&dek, &server_kek);
        let encrypted_email_kek = encrypt_data(&user.email, &server_kek);
        let encrypted_uid = encrypt_data(&new_user.uid.to_string(), &server_kek);

        let dek_data = Dek::new_dek(&encrypted_uid, &encrypted_email_kek, &encrypted_dek);

        // add the dek to the database
        db.collection("deks")
            .insert_one(dek_data, None)
            .await
            .unwrap();

        println!(">> {:?} added. uid: {:?}", new_user.name, new_user.uid);
    }
}
