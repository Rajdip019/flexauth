
# User Data Protection

Data protection is one of the main things for a auth server and we have taken that seriously. Here is a breif how the data gets encrypted and stored in the database.



## Method

The method we are using for encryption is **Envelope Encryption**

### Terminology ( to keep in mind )
- `DEK`: Data Encryption Key
- `KEK` : Key Encryption Key


## Diagram

![data-protection-inhouse-auth](https://github.com/Rajdip019/in-house-auth/assets/91758830/163fdd5a-1757-481c-ba18-3a4bfacb72d2)


## Explaination

Here is a strp by step guide on how it works.

### Step 1:
Every user is assigned a new and unique `DEK` when they sign up. 

### Step 2: 
We encrypt all the user data from `Session Details`, `Password Reset Request` and all with the user `DEK` using the `AESGcm256` algorithm and store it to DB.

### Step 3:
The auth server has his own `KEK`. This is unique for the server. You can generate it by running the command below from the root of your project. ( Make sure you have cargo installed ) - [How to install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```
cargo run --bin create_kek 
```

### Step 4:
We use the `KEK` to encrypt the `DEK` using the same `AESGcm256` algorithm and store it to DB.

### Step 5: ( Additional )
For additional safety you can use `GCP KMS`, `AWS KMS` or any other cloud provider for additional safety. 



## Feedback

If you have any feedback, please raise a issue or start a discussion. Thank you.
