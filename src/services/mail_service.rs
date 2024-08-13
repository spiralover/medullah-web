use std::collections::HashMap;

use chrono::{Datelike, Utc};
use futures_util::TryFutureExt;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tera::Context;
use tokio::spawn;

use crate::helpers::once_lock::OnceLockHelper;
// use crate::models::user::{FullName, UserMinimalData};
use crate::prelude::AppMessage;
use crate::results::AppResult;
use crate::MEDULLAH;

#[derive(Serialize, Deserialize, Clone)]
pub struct Mailbox {
    pub name: String,
    pub email: String,
}

impl Mailbox {
    pub fn new(name: &str, email: &str) -> Mailbox {
        Mailbox {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    // pub fn from_minimal_user(u: &UserMinimalData) -> Mailbox {
    //     Mailbox::new(u.full_name().as_str(), u.email.as_str())
    // }
}

#[derive(Deserialize, Debug, Clone)]
pub struct MailerResponse {
    pub code: i16,
    pub success: bool,
    pub status: String,
    pub message: String,
    pub data: HashMap<String, String>,
}

#[derive(Debug)]
pub struct MailerError {
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct MailerService {
    for_each_recv: bool,
    receiver: Vec<Mailbox>,
    cc: Vec<Mailbox>,
    bcc: Vec<Mailbox>,
    reply_to: Vec<Mailbox>,
    from: Mailbox,
    subject: String,
    message: String,
}

#[derive(Serialize)]
struct MailPayload {
    app: String,
    mails: Vec<MailerService>,
}

impl Default for MailerService {
    fn default() -> Self {
        MailerService {
            for_each_recv: false,
            cc: vec![],
            bcc: vec![],
            reply_to: vec![],
            receiver: vec![],
            message: String::from(""),
            subject: String::from(""),
            from: Mailbox {
                name: MEDULLAH.app().mailer_config.from_name.clone(),
                email: MEDULLAH.app().mailer_config.from_email.clone(),
            },
        }
    }
}

impl MailerService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message(ctx: Context) -> MailerService {
        let mut srv = Self::new();
        srv.view("message", ctx);
        srv
    }

    pub fn context() -> Context {
        Context::new()
    }

    pub fn subject(&mut self, s: String) -> &mut MailerService {
        self.subject = s;
        self
    }

    pub fn subject_titled(&mut self, s: &str) -> &mut MailerService {
        self.subject = MEDULLAH.app().title(s);
        self
    }

    pub fn for_each_recv(&mut self) -> &mut MailerService {
        self.for_each_recv = true;
        self
    }

    pub fn receivers(&mut self, r: Vec<Mailbox>) -> &mut MailerService {
        self.receiver = r;
        self
    }

    pub fn body(&mut self, b: String) -> &mut MailerService {
        self.message = b;
        self
    }

    pub fn view(&mut self, file: &str, mut ctx: Context) -> &mut MailerService {
        let app = MEDULLAH.app();
        ctx.insert("year", &Utc::now().year());
        ctx.insert("app_name", &app.app_name.clone());
        ctx.insert("app_desc", &app.app_desc.clone());
        ctx.insert("app_help_email", &app.app_help_email.clone());
        ctx.insert("app_frontend_url", &app.app_frontend_url.clone());

        self.body(app.render(file.to_string(), ctx))
    }

    pub fn send_silently(&mut self) {
        let mut mailer = self.clone();
        spawn(async move { mailer.send().await });
    }

    pub async fn send(&mut self) -> AppResult<MailerResponse> {
        let max_loop = 3;
        for i in 0..max_loop {
            let response = self.do_send().await;
            if let Ok(resp) = response {
                match resp.0.success {
                    true => info!("[mailer-response]: {}", resp.1),
                    false => error!("[mailer-error]: {}", resp.1),
                };

                return Ok(resp.0);
            }

            let error = response.unwrap_err();

            error!("mailer error: {:?}", error);

            if i == max_loop {
                return Err(error);
            }
        }

        // We shouldn't reach here, but let's make Rust happy :)
        Err(AppMessage::WarningMessage("something went wrong"))
    }

    async fn do_send(&self) -> AppResult<(MailerResponse, String)> {
        debug!("sending '{}'...", self.subject);

        let client = reqwest::Client::new();
        let address = format!(
            "{}/api/v1/applications/{}/mails",
            MEDULLAH.app().mailer_config.server_endpoint,
            MEDULLAH.app().mailer_config.server_application_id,
        );

        let payload = match self.for_each_recv {
            true => {
                let mut mails = vec![];
                for receiver in &self.receiver {
                    let mut rec = self.clone();
                    rec.receiver = vec![receiver.clone()];
                    mails.push(rec);
                }

                MailPayload {
                    app: "tirob".to_string(),
                    mails,
                }
            }
            false => MailPayload {
                app: "tirob".to_string(),
                mails: vec![self.clone()],
            },
        };

        let resp = client
            .post(address)
            .json(&payload)
            .bearer_auth(MEDULLAH.app().mailer_config.server_auth_token.clone())
            .send()
            .map_err(AppMessage::MailerError)
            .await?
            .text()
            .map_err(AppMessage::MailerError)
            .await?;

        Ok((serde_json::from_str(&resp)?, resp))
    }
}
