use super::Authentication;
use super::endpoint;
use super::response;
use super::response::{ApiRequestResult};
use super::http_client::{HttpClient, RequestContext, MethodType, HeaderType};
use super::error::PixelaClientError;

use std::collections::HashMap;
use failure::Error;
use serde_json;
use serde_json::{Number};

/// Data representing the quantity of each day.
#[derive(Serialize, Deserialize, Debug)]
pub struct Pixel {
    /// The date on which the quantity is to be recorded. It is specified in yyyyMMdd format.
    pub date: String,
    /// Specify the quantity to be registered on the specified date.
    /// Validation rule: int^\-?[0-9]+ float^\-?[0-9]+\.[0-9]+
    pub quantity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PixelQuantity {
    pub quantity: Number,
}

pub(crate) fn create<T>(auth: &Authentication, graph_id: &str, pixel: &Pixel) -> Result<(), Error> where T: HttpClient {
    let body = serde_json::to_string(pixel)?;
    let uri = endpoint::graph(&auth.username, graph_id);
    let context = RequestContext::new(
        &uri,
        MethodType::Post,
        Some(&body),
        Some(&auth.token),
    );

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

pub(crate) fn update<T>(auth: &Authentication, graph_id: &str, pixel: &Pixel) -> Result<(), Error> where T: HttpClient {
    let mut hash: HashMap<&str, &str> = HashMap::new();
    hash.insert("quantity", &pixel.quantity);

    let body = serde_json::to_string(&hash)?;
    let uri = endpoint::pixel(&auth.username, graph_id, &pixel.date);
    let context = RequestContext::new(
        &uri,
        MethodType::Put,
        Some(&body),
        Some(&auth.token),
    );

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

pub(crate) fn delete<T>(auth: &Authentication, graph_id: &str, date: &str) -> Result<(), Error> where T: HttpClient {
    let uri = endpoint::pixel(&auth.username, graph_id, date);
    let context = RequestContext::new(
        &uri,
        MethodType::Delete,
        None,
        Some(&auth.token),
    );

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

pub(crate) fn get<T>(auth: &Authentication, graph_id: &str, date: &str) -> Result<Pixel, Error> where T: HttpClient {
    let uri = endpoint::pixel(&auth.username, graph_id, date);
    let context = RequestContext::new(
        &uri,
        MethodType::Get,
        None,
        Some(&auth.token),
    );

    let body = T::do_request(&context)?;

    let res: Result<ApiRequestResult, _> = serde_json::from_str(&body);
    if let Ok(v) = res {
        return Err(PixelaClientError::RequestNotSuccess(v.message).into());
    }

    let res: PixelQuantity = serde_json::from_str(&body)?;
    Ok(Pixel {
        date: date.to_owned(),
        quantity: res.quantity.to_string(),
    })
}

pub(crate) fn increment<T>(auth: &Authentication, graph_id: &str) -> Result<(), Error> where T: HttpClient {
    let uri = endpoint::increment(&auth.username, graph_id);
    let mut context = RequestContext::new(
        &uri,
        MethodType::Put,
        None,
        Some(&auth.token),
    );

    context.insert_header(HeaderType::ContentLength, "0");

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

pub(crate) fn decrement<T>(auth: &Authentication, graph_id: &str) -> Result<(), Error> where T: HttpClient {
    let uri = endpoint::decrement(&auth.username, graph_id);
    let mut context = RequestContext::new(
        &uri,
        MethodType::Put,
        None,
        Some(&auth.token),
    );

    context.insert_header(HeaderType::ContentLength, "0");

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

#[cfg(test)]
mod pixel_test {
    use super::*;

    #[test]
    fn pixel_quantity_deserialize_test() {
        let res: Result<PixelQuantity, _> = serde_json::from_str(r#"{"quantity":50}"#);
        if let Ok(v) = res {
            assert_eq!(v.quantity.to_string(), "50");
        } else {
            panic!("failed serialize params");
        };
    }
}