use serde::Serialize;
use std::collections::HashMap;
use temporal_sdk_core::protos::temporal::api::common::v1::Payload;

pub fn create_payload(content: impl Serialize) -> Payload {
    let mut metadata = HashMap::new();
    metadata.insert("encoding".to_owned(), "json/plain".as_bytes().to_vec());

    let data = match serde_json::to_string(&content) {
        Ok(data) => data,
        Err(_) => "".to_string(),
    };

    Payload {
        metadata: metadata.to_owned(),
        data: data.into(),
    }
}
