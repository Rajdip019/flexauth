
## Local Setup

### Step 1 : Pre Requisities
- [Rust Basics](https://doc.rust-lang.org/book/)
- [Cargo (Rust package manager)](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Docker (For containerization)](https://docs.docker.com/get-docker/)
- [MongoDB Compass (For Visualising DB with GUI)](https://www.mongodb.com/try/download/compass)


### Step 2: Environment Variables

To run this project, you will need to add the following environment variables to your .env file. Have a look at the `.env.example` file to see all the keys needed.


You can generate `SERVER_KEK` by running the command below from the root of your project. ( Make sure you have cargo installed
```
cargo run --bin create_kek

```


SMTP servers require authentication to ensure that only authorized users can send emails. For generating `EMAIL_PASSWORD`, Visit this [link](https://support.google.com/mail/thread/205453566/how-to-generate-an-app-password?hl=en).
`SMTP_DOMAIN = smtp.gmail.com` as we are using GMAIL as a Mail Provider.


Also we need a Private Key for verifying Sessions (Make sure you have openssl in your system) and place it at the root of the project.
```
openssl genpkey -algorithm RSA -out private_key.pem -aes256 -pass pass:your_passphrase -pkeyopt rsa_keygen_bits:2048

```

### Step 3: Spinning up Docker Containers

Not it's time to run the docker container by running this following command (Make sure you have Docker installed)
```
docker compose up

```

Note:- If there's any changes outside of the `/src` directory like- `cargo.toml` file, Make sure to stop the container and run the docker container with `--build` flag
```
docker compose up --build

```

Congrats, Your Local Setup is done successfully.





