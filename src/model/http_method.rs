use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}
