use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub statuscode: StatusCode,
    pub payload: Value,
}

#[derive(Debug, Serialize)]
pub enum StatusCode {
    Ok,
    RequestFailure,
    InternalServerError,
}

impl<'de> serde::Deserialize<'de> for StatusCode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<StatusCode, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StatusCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a statuscode as integer or text")
            }

            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(match value {
                    200 => StatusCode::Ok,
                    400 => StatusCode::RequestFailure,
                    500 => StatusCode::InternalServerError,
                    _ => {
                        return Err(E::invalid_value(
                            serde::de::Unexpected::Unsigned(value),
                            &self,
                        ))
                    }
                })
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(match value {
                    "200" => StatusCode::Ok,
                    "400" => StatusCode::RequestFailure,
                    "500" => StatusCode::InternalServerError,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Str(value), &self)),
                })
            }
        }

        d.deserialize_any(Visitor)
    }
}
