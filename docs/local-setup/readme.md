## Local Setup

### Step 1: Pre-requisites

- [Rust Basics](https://doc.rust-lang.org/book/)
- [Cargo (Rust package manager)](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Docker (For containerization)](https://docs.docker.com/get-docker/)
- [MongoDB Compass (For Visualising DB with GUI)](https://www.mongodb.com/try/download/compass)

### Step 2: Environment Variables

Run this command to start setting up the environment👇
```
make setup
```
It will automatically start asking you all the required environment variables it needs and automatically create the environment variables it can.

#### Here are some of the variables you need and here's how to get them 👇
You can generate `SERVER_KEK` by running the command below from the root of your project. ( Make sure you have cargo installed )

```
cargo run --bin create_kek

```
For testing purposes only you can use this SERVER_KEK as well: **9628177f62a03f5db4742273b915bf66.a21a897aa750**

SMTP servers require authentication to ensure that only authorized users can send emails. For generating `EMAIL_PASSWORD`, Visit this [link](https://support.google.com/mail/thread/205453566/how-to-generate-an-app-password?hl=en).
`SMTP_DOMAIN = smtp.gmail.com` as we are using GMAIL as a Mail Provider.

For MongoDB username and password, you can use anything you want. But remember you need the same username and password for Mongo Compass mentioned in step 4.

### Step 3: Spinning up Docker Containers

Now it's time to run the docker container by running this following command (Make sure you have Docker installed)

If you want to build the containers and then want to spin them up use this command 👇 
```
make build-run-server
```

Otherwise, you can use 
```
make run-server
```

This command will start the container and watch the /src folder for any changes. If there are any modifications to the content inside /src, the container will automatically hot reload to reflect those changes.

Note:- If there's any changes outside of the `/src` directory like- `cargo.toml` file, Make sure to stop the container and run the make command with `make build-run-server`.


### Step 4: Connecting to MongoDB Compass

After running the Docker containers, you can connect to the MongoDB database using MongoDB Compass. Follow these steps:

1. **Open MongoDB Compass**: Launch MongoDB Compass on your system.

2. **Connect to a MongoDB Deployment**:
   - Click on the "New Connection" button to create a new connection.
   - In the "Connection String" field, paste the following URI:
     ```plaintext
     mongodb://${MONGO_INITDB_ROOT_USERNAME}:${MONGO_INITDB_ROOT_PASSWORD}@localhost:27017/?directConnection=true&retryWrites=true&w=majority
     ```
   - Replace the default URI with the one appropriate for your setup if necessary. This URI includes the credentials (`${MONGO_INITDB_ROOT_USERNAME}:${MONGO_INITDB_ROOT_PASSWORD}`) from the .env and the default MongoDB port (`27017`).
   
3. **Connect to the Database**:
   - Click on the "Connect" button to establish a connection to the MongoDB deployment.
   - If the connection is successful, you will be able to browse and interact with the databases and collections in your MongoDB instance using MongoDB Compass.

4. **Explore Data**: You can now explore your MongoDB databases, collections, and documents, run queries, and perform other operations using MongoDB Compass.

That's it! You are now connected to your MongoDB database using MongoDB Compass.

### Step 5: Running the UI:
You just need to run this command 👇
```
make run-ui
```

Congrats, Your Local Setup is done successfully.
