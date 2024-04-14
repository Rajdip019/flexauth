use mongodb::{Client, Collection};
use std::env;

use crate::{
    models::user_model::User,
    utils::{
        dek_utils::new_dek,
        encryption_utils::{create_dek, encrypt_data},
        hashing_utils::salt_and_hash_password,
        user_utils::new_user,
    },
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
        let dek = create_dek();
        // hash and salt the password
        let hashed_and_salted_pass = salt_and_hash_password(&user.password);

        // encrypt sensitive data
        let encrypted_password = encrypt_data(&hashed_and_salted_pass.password, &dek);
        let encrypted_salt = encrypt_data(&hashed_and_salted_pass.salt, &dek);
        let formatted_pass_with_salt = format!("{}.{}", encrypted_password, encrypted_salt);
        let encrypted_email = encrypt_data(&user.email, &dek);
        let encrypted_role = encrypt_data(&user.role, &dek);

        // create a new user
        let new_user = new_user(
            user.name,
            encrypted_email,
            encrypted_role,
            formatted_pass_with_salt,
        );

        // add user to the database
        let db = mongo_client.database("test");
        db.collection("users")
            .insert_one(new_user.clone(), None)
            .await
            .unwrap();

        // encrypt the data with kek
        let server_kek = env::var("SERVER_KEK").expect("Server Kek must be set.");
        let encrypted_dek = encrypt_data(&dek, &server_kek);
        let encrypted_email_kek = encrypt_data(&user.email, &server_kek);
        let encrypted_uid = encrypt_data(&new_user.uid.to_string(), &server_kek);

        let dek_data = new_dek(encrypted_uid, encrypted_email_kek, encrypted_dek);

        // add the dek to the database
        db.collection("deks")
            .insert_one(dek_data, None)
            .await
            .unwrap();

        println!(">> {:?} added. uid: {:?}", new_user.name, new_user.uid);
    }
}
