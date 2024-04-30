use std::env;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct Email {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub async fn send_email(&self) {
        let smtp_username = env::var("EMAIL").expect("EMAIL_ID must be set");
        let smtp_password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");
        let name = env::var("MAIL_NAME").expect("NAME must be set");
        let smtp_domain = env::var("SMTP_DOMAIN").expect("SMTP_DOMAIN must be set");

        let from = format!("{} <{}>", name, smtp_username);
        let to = format!("{} <{}>", self.name, self.email);
        let message = Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(self.subject.to_owned()) // Convert &str to String
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(self.body.to_owned()))
            .unwrap();

        let credentials = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&smtp_domain)
            .unwrap()
            .credentials(credentials)
            .build();

        // Send the email
        match mailer.send(&message) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}
