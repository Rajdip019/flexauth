
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;

pub async fn connect() -> Result<Client, Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =  env::var("MONGO_URI").unwrap_or("mongodb://admin:admin@localhost:27017/?directConnection=true&retryWrites=true&w=majority".to_string());
        

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;

    let client = Client::with_options(options.clone())?;
    // Print success message if the connection is successful or an error message if it fails:
    // test the connection to the database
    match client.list_database_names(None, None).await {
        Ok(_) =>{
            println!(">> Successfully connected to the database");
            Ok(client)
        },
        Err(e) => {
            eprintln!(">> Error connecting to the database: {:?}", e);
            std::process::exit(1);
        },
    }
}
