use super::Authentication;
use super::endpoint;
use super::response;
use super::http_client::{HttpClient, RequestContext, MethodType};

use failure::Error;
use std::collections::HashMap;
use serde_json;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserParam {
    pub token: String,
    pub username: String,
    pub agree_terms_of_service: ConsentAnswer,
    pub not_minor: ConsentAnswer,
}

/// Answer to the consent from. yes or no.
#[derive(Serialize, Deserialize)]
pub enum ConsentAnswer {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
}

pub(crate) fn create<T>(param: &CreateUserParam) -> Result<(), Error> where T: HttpClient {
    let uri = endpoint::users();
    let body = serde_json::to_string(param)?;
    let context = RequestContext::new(
        &uri,
        MethodType::Post,
        Some(&body),
        None,
    );

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

pub(crate) fn update<T>(auth: &Authentication, new_token: &str) -> Result<(), Error> where T: HttpClient {
    let mut hash: HashMap<&str, &str> = HashMap::new();
    hash.insert("newToken", new_token);

    let uri = endpoint::user(&auth.username);
    let body = serde_json::to_string(&hash)?;
    let context = RequestContext::new(
        &uri,
        MethodType::Put,
        Some(&body),
        Some(&auth.token),
    );

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

pub(crate) fn delete<T>(auth: &Authentication) -> Result<(), Error> where T: HttpClient {
    let uri = endpoint::user(&auth.username);
    let context = RequestContext::new(
        &uri,
        MethodType::Delete,
        None,
        Some(&auth.token),
    );

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

#[cfg(test)]
mod user_test {
    use super::*;

    #[test]
    fn create_user_param_serialize_test() {
        let param = CreateUserParam {
            token: "testtoken".to_owned(),
            username: "testuser".to_owned(),
            agree_terms_of_service: ConsentAnswer::Yes,
            not_minor: ConsentAnswer::No,
        };

        let res = serde_json::to_string(&param);
        if let Ok(v) = res {
            assert_eq!(v, r#"{"token":"testtoken","username":"testuser","agreeTermsOfService":"yes","notMinor":"no"}"#);
        } else {
            panic!("failed serialize params");
        };
    }
}