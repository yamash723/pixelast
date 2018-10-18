use super::PixelaClientError;

use serde_json;
use failure::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiRequestResult {
    pub message: String,
    pub is_success: bool,
}

pub fn build_result(json: &str) -> Result<(), Error> {
    let res: ApiRequestResult = serde_json::from_str(&json)?;

    if !res.is_success {
        return Err(PixelaClientError::RequestNotSuccess(res.message).into());
    }

    Ok(())
}

#[cfg(test)]
mod response_test {
    use super::*;

    #[test]
    fn build_result_is_success_test() {
        let body = r#"{"message":"success message","isSuccess":true}"#;
        let res = build_result(&body);

        if let Err(e) = res {
            panic!("failed build result. {}", e);
        };
    }

    #[test]
    fn build_result_is_not_success_test() {
        let body = r#"{"message":"failed message","isSuccess":false}"#;
        let res = build_result(&body);

        if let Ok(_) = res {
            panic!("not failed.");
        };
    }

    #[test]
    fn build_result_is_not_json_body_test() {
        let body = "not json body";
        let res = build_result(&body);

        if let Ok(_) = res {
            panic!("not failed.");
        };
    }
}