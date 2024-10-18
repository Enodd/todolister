use serde::Deserialize;

pub enum ResponseType {
    Success,
    Data,
    Error
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Result<T> {
    Success(String),
    Data(T),
    Error { code: u16, message: String }
}

pub struct Response<T> (Result<T>);

impl<T> Response<T> {
    pub fn new(
        response_type: ResponseType,
        message: &str,
        code: u16,
        data: Option<T>
    ) -> Response<T> {
        match response_type {
            ResponseType::Success => Self (Result::Success(message.to_owned())),
            ResponseType::Data => match data {
                Some(data ) => Self(Result::Data(data)),
                None => Self(Result::Error { code, message: message.to_owned() })
            },
            ResponseType::Error => Self(Result::Error { code, message: message.to_owned() })
        }
    }
}