use std::fmt::{self};

use serde::{Deserialize, Serialize, de::Error};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Serialize for HttpMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HttpMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let method = String::deserialize(deserializer)?;
        let method = method.to_uppercase();
        let method = method.as_str();

        let method = match method {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "PATCH" => HttpMethod::Patch,
            "DELETE" => HttpMethod::Delete,
            _ => {
                return Err(Error::invalid_type(
                    serde::de::Unexpected::Str(method),
                    &"one of GET, POST, PUT, PATCH and DELETE.",
                ));
            }
        };

        Ok(method)
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        formatter.write_str(&self.to_string())
    }
}

impl HttpMethod {
    pub fn to_string(&self) -> String {
        let slice = match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        };

        slice.to_string()
    }
}
