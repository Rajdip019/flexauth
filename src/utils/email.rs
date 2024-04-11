use std::env;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct Email{
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
}

pub async fn send_email(email: Email) {
    let smtp_username = env::var("EMAIL").expect("EMAIL_ID must be set");
    let smtp_password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");
    let name = env::var("MAIL_NAME").expect("NAME must be set");
    let smtp_domain = env::var("SMTP_DOMAIN").expect("SMTP_DOMAIN must be set");

    let from = format!("{} <{}>", name, smtp_username);
    let to = format!("{} <{}>", email.name, email.email);
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(email.subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(email.body))
        .unwrap();

    let credentials = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&smtp_domain)
        .unwrap()
        .credentials(credentials)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
