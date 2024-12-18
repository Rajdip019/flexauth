use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;

pub async fn connect() -> Result<Client, Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri_main = env::var("MONGO_URI").unwrap_or("localhost".to_string());

    let client_uri = format!(
        "mongodb://{}:{}@{}:27017/?directConnection=true&retryWrites=true&w=majority",
        env::var("MONGO_INITDB_ROOT_USERNAME").expect("MONGO_INITDB_ROOT_USERNAME required"),
        env::var("MONGO_INITDB_ROOT_PASSWORD").expect("MONGO_INITDB_ROOT_PASSWORD required"),
        client_uri_main
    );

    let options =
        match ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
        {
            Ok(options) => options,
            Err(e) => {
                eprintln!(">> Error parsing client options: {:?}", e);
                std::process::exit(1);
            }
        };

    let client = Client::with_options(options.clone())?;
    // Print success message if the connection is successful or an error message if it fails:
    // test the connection to the database
    match client.list_database_names(None, None).await {
        Ok(_) => {
            println!(">> Successfully connected to the database");
            Ok(client)
        }
        Err(e) => {
            eprintln!(">> Error connecting to the database: {:?}", e);
            std::process::exit(1);
        }
    }
}
