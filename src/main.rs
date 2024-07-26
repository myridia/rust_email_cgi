/*
 https://docs.rs/cgi/latest/cgi/
*/

extern crate cgi;
use lettre::message::{header, header::ContentType, Attachment, Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransportBuilder;
use lettre::{SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::io::{Error, ErrorKind, Read, Result};

fn main() {
    // curl --data "id=value1" --data "name=value2" http://127.0.0.1/email.cgi
    cgi::handle(|req: cgi::Request| -> cgi::Response {
        let h = req.headers();
        let b = req.body();
        let rf = &h.get("referer").unwrap().to_str().unwrap();
        let mut ob = serde_urlencoded::from_bytes::<Vec<(String, String)>>(b);
        let res = email(ob.unwrap());
        let r = format!(
            "<html><body><div>{1}</div><h1>Successfully send email.</h1><a href='{0}'>go back to:  {0} </a></body></html>",
            &rf,res
        );
        cgi::html_response(200, r)
    })
}

fn email(obj: Vec<(String, String)>) -> String {
    let mut to_mail = "".to_string();
    let mut subject = "".to_string();
    let mut body = "".to_string();
    let mut csv_header = "".to_string();
    let mut csv_body = "".to_string();
    let mut csv = "".to_string();
    let mut filename = "".to_string();

    for i in obj {
        body.push_str(&format!("{0} : {1} \n", &i.0, &i.1));
        csv_header.push_str(&format!("{0},", &i.0));
        csv_body.push_str(&format!("\"{0}\",", &i.1));
        //println!("{:?}", i);
        if "mail" == &i.0 {
            to_mail = i.1.to_string();
        }
        if "id" == &i.0 {
            subject = format!("csv added for {0}", &i.1);
            filename = format!("csv_{0}.csv", &i.1);
        }
    }
    body.push_str(&format!("sender : Email CCI "));
    if to_mail != "" && filename != "" {
        csv.push_str(&csv_header);
        csv.push_str("\n");
        csv.push_str(&csv_body);

        let content_type = ContentType::parse("text/csv").unwrap();
        let attachment = Attachment::new(filename).body(csv.to_string(), content_type);

        let email = Message::builder()
            .from("Foo <paradise@perege.com>".parse().unwrap())
            .to(format!("{to_mail} <{to_mail}>").parse().unwrap())
            .subject(subject)
            .multipart(
                MultiPart::mixed()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(body.to_string()),
                    )
                    .singlepart(attachment),
            )
            .unwrap();

        let creds = Credentials::new("foo@bar.com".to_string(), "barpass".to_string());
        let mailer = SmtpTransport::starttls_relay("foofighter.com")
            .unwrap()
            .credentials(creds)
            .port(26)
            .build();

        match mailer.send(&email) {
            Ok(_e) => {
                return "Ok".to_string();
            }
            Err(_e) => {
                let ret = format!("Failed - {:?}", _e).to_string();
                return ret;
            }
        }
    } else if to_mail != "" {
        let email = Message::builder()
            .from("Foo <foo@bar.com>".parse().unwrap())
            .to(format!("{to_mail} <{to_mail}>").parse().unwrap())
            .subject("Newletter Subscriber")
            .body(body.to_string())
            .unwrap();

        let creds = Credentials::new("foo@bar.com".to_string(), "foobar".to_string());
        let mailer = SmtpTransport::starttls_relay("foofighter.com")
            .unwrap()
            .credentials(creds)
            .port(26)
            .build();

        match mailer.send(&email) {
            Ok(_e) => {
                return "Ok".to_string();
            }
            Err(_e) => {
                let ret = format!("Failed - {:?}", _e).to_string();
                return ret;
            }
        }
    }
    return "Failed".to_string();
}
