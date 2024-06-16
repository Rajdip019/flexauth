## Local Setup

### Step 1 : Pre Requisities

- [Rust Basics](https://doc.rust-lang.org/book/)
- [Cargo (Rust package manager)](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Docker (For containerization)](https://docs.docker.com/get-docker/)
- [MongoDB Compass (For Visualising DB with GUI)](https://www.mongodb.com/try/download/compass)

### Step 2: Environment Variables

To run this project, you will need to add the following environment variables to your .env file. Have a look at the `.env.example` file to see all the keys needed.

You can generate `SERVER_KEK` by running the command below from the root of your project. ( Make sure you have cargo installed )

```
cargo run --bin create_kek

```

SMTP servers require authentication to ensure that only authorized users can send emails. For generating `EMAIL_PASSWORD`, Visit this [link](https://support.google.com/mail/thread/205453566/how-to-generate-an-app-password?hl=en).
`SMTP_DOMAIN = smtp.gmail.com` as we are using GMAIL as a Mail Provider.

You have to add a random Access Key `X_API_KEY` for secure authentication of the endpoints.
Alternatively you can use this command to generate a random `API KEY`

```shell
openssl rand -hex 32
```

Also we need a Private Key for verifying Sessions (Make sure you have openssl in your system) and place it at the root of the project.

```
openssl genpkey -algorithm RSA -out private_key.pem -pkeyopt rsa_keygen_bits:2048

```

### Step 3: Spinning up Docker Containers

Now it's time to run the docker container by running this following command (Make sure you have Docker installed)

```
docker compose up

```
This command will start the container and watch the /src folder for any changes. If there are any modifications to the content inside /src, the container will automatically hot reload to reflect those changes.

Note:- If there's any changes outside of the `/src` directory like- `cargo.toml` file, Make sure to stop the container and run the docker container with `--build` flag

```
docker compose up --build

```

### Step 4: Connecting to MongoDB Compass

After running the Docker containers, you can connect to the MongoDB database using MongoDB Compass. Follow these steps:

1. **Open MongoDB Compass**: Launch MongoDB Compass on your system.

2. **Connect to a MongoDB Deployment**:
   - Click on the "New Connection" button to create a new connection.
   - In the "Connection String" field, paste the following URI:
     ```plaintext
     mongodb://admin:admin@localhost:27017/?directConnection=true&retryWrites=true&w=majority
     ```
   - Replace the default URI with the one appropriate for your setup if necessary. This URI includes the credentials (`admin:admin`) and the default MongoDB port (`27017`).
   
3. **Connect to the Database**:
   - Click on the "Connect" button to establish a connection to the MongoDB deployment.
   - If the connection is successful, you will be able to browse and interact with the databases and collections in your MongoDB instance using MongoDB Compass.

4. **Explore Data**: You can now explore your MongoDB databases, collections, and documents, run queries, and perform other operations using MongoDB Compass.

That's it! You are now connected to your MongoDB database using MongoDB Compass.


Congrats, Your Local Setup is done successfully.
