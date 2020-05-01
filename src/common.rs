use serde::Serialize;

use crate::session::{Session, CommonResponse};
use crate::message::MessageId;
use crate::error::{Result, assert};

/// Others
impl Session {
    pub async fn recall(&self, message_id: MessageId) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: MessageId,
        }

        let req = Request {
            session_key: self.key.clone(),
            target: message_id,
        };

        let resp: CommonResponse = self.client.post(&self.url("/recall"))
            .json(&req).send().await?
            .json().await?;

        assert(resp.code, "Recall")
    }
}