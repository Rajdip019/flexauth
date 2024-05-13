
# Password protection

Password protection is another core functionality for an auth server and we have taken that seriously. Here is a brief on how the password gets salted, hashed, encrypted, and stored in the database.



## Method

They are using multiple hashing algorithms for **protecting passwords**.

### Hashing Algorithms used.
- `Agron`: Hasing and salting - [carte link](https://crates.io/crates/argon2)
- `Sha256` : Final Hashing - [carte link](https://crates.io/crates/sha256)


## Diagram

![password-protection-inhouse-auth](https://github.com/Rajdip019/in-house-auth/assets/91758830/bdee629b-ea6f-4a61-b3a2-989e0fcf2a11)


## Explaination

Here is a step-by-step guide on how it works.

### Step 1:
Raw Password is salted using a random salt created by the `Argon` library. And then hashed which looks like this -

`$argon2i$v=19$m=16,t=2,p=1$S0FVN2NRbHF2RzBzOXBLSg$xLmzkDhV/z9qRPLpD2ybqw`

You can generate and play with argon hashing library configurations here - [Argon2 online](https://argon2.online/)

### Step 2:
The Agron hash is then put into `sha256` again to generate the more random and more random hash. It gives a 256bit hex which looks like this -  `16567bc6bf75f0ac224749b27b42487012246768dbb8bce95b8638b6ab826ca01`

### Step 3: 
We encrypt the password using the user `DEK` using the `AESGcm256` algorithm and store it in DB this ensures a higher level of secure and unique hex string.

### Step 3:
The auth server has its own `KEK`. This is unique for the server. You can generate it by running the command below from the root of your project. ( Make sure you have cargo installed ) - [How to install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```

cargo run --bin create_kek

```

### Step 4:
We use the `KEK` to encrypt the `DEK` using the same `AESGcm256` algorithm and store it in DB.

### Step 5: ( Additional )
For additional safety, you can use `GCP KMS`, `AWS KMS` or any other cloud provider for additional safety. 



## Feedback

If you have any feedback, please raise an issue or start a discussion. Thank you.

