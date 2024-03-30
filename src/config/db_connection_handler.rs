
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
// use std::env;
use std::error::Error;

pub async fn mongo_connection_handler() -> Result<Client, Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        "mongodb://admin:admin@localhost:27017/?directConnection=true&retryWrites=true&w=majority";

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;

    let client = Client::with_options(options.clone())?;
    // Print success message if the connection is successful or an error message if it fails:
    match Client::with_options(options) {
        Ok(_) => println!("Successfully connected to MongoDB!"),
        Err(e) => eprintln!("Error connecting to MongoDB: {}", e),
    }
    // return client;
    return Ok(client);
}
