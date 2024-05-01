use mongodb::{Client, Collection};

use crate::core::{user::User, dek::Dek};

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
        let new_user = User::new(&user.name, &user.email, &user.role, &user.password);
        let dek = Dek::generate();
        match new_user.encrypt_and_add(&mongo_client, &dek).await {
            Ok(_) => {},
            Err(e) => {
                println!(">> Error adding user: {:?}", e);
                continue;
            }
        };

        // add the dek to the deks collection
        match Dek::new(&new_user.uid, &new_user.email, &dek).encrypt_and_add(&mongo_client).await {
            Ok(_) => {}
            Err(e) => {
                println!(">> Error adding dek: {:?}", e);
                continue;
            }
        }

        println!(">> {:?} added. uid: {:?}", new_user.name, new_user.uid);
    }
}
