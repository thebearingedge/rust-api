use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    title: String,
    body: String,
    author: String,
    date: DateTime<Utc>,
    post_id: Uuid,
}

impl Post {
    pub fn new(title: &str, body: &str, author: &str, date: DateTime<Utc>, post_id: Uuid) -> Post {
        Post {
            title: title.to_string(),
            body: body.to_string(),
            author: author.to_string(),
            date,
            post_id,
        }
    }

    pub fn post_id(&self) -> &Uuid {
        &self.post_id
    }
}
