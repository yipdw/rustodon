use serde::Serialize;
use serde_json::{self, Value};
use rocket::Request;
use rocket::http::{Status, ContentType};
use rocket::response::{self, Responder, Content};
use ::db::models::Account;

pub struct ActivityStreams<T = Value>(pub T);

impl<T> Responder<'static> for ActivityStreams<T>
    where T: Serialize
{
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        serde_json::to_string(&self.0).map(|string| {
            let ap_json = ContentType::new("application", "activity+json");

            Content(ap_json, string).respond_to(req).unwrap()
        }).map_err(|e| {
            // TODO: logging
            Status::InternalServerError
        })
    }
}

pub trait AsActivityPub {
    fn as_activitypub(&self) -> ActivityStreams;
}

impl AsActivityPub for Account {
    fn as_activitypub(&self) -> ActivityStreams<serde_json::Value> {
        ActivityStreams(json!({
            "@context": "https://www.w3.org/ns/activitystreams",
            "type": "Person",
            "id": self.get_uri(),

            "inbox": self.get_inbox_endpoint(),
            "outbox": self.get_outbox_endpoint(),

            "following": self.get_following_endpoint(),
            "followers": self.get_followers_endpoint(),

            "preferredUsername": self.username,
            "name": self.display_name.as_ref().map(String::as_str).unwrap_or(""),
            "summary": self.summary.as_ref().map(String::as_str).unwrap_or("<p></p>"),
        }))
    }
}
