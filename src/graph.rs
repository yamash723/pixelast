use super::Authentication;
use super::endpoint;
use super::response;
use super::response::{ApiRequestResult};
use super::http_client::{HttpClient, RequestContext, MethodType};
use super::error::PixelaClientError;

use failure::Error;
use serde_json;

/// Graph definition in Pixela.
#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    /// It is an ID for identifying the pixelation graph.
    /// Validation rule: ^[a-z][a-z0-9-]{1,16}
    pub id: String,
    /// It is the name of the pixelation graph.
    pub name: String,
    /// It is a unit of the quantity recorded in the pixelation graph. Ex. commit, kilogram, calory.
    pub unit: String,
    /// It is the type of quantity to be handled in the graph.
    #[serde(rename = "type")]
    pub graph_type: GraphType,
    /// Defines the display color of the pixel in the pixelation graph.
    pub color: GraphColor,
}

/// It is the type of quantity to be handled in the graph.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum GraphType {
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "float")]
    Float,
}

/// Defines the display color of the pixel in the pixelation graph.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum GraphColor {
    #[serde(rename = "shibafu")]
    Shibafu,
    #[serde(rename = "emoji")]
    Emoji,
    #[serde(rename = "sora")]
    Sora,
    #[serde(rename = "ichou")]
    Ichou,
    #[serde(rename = "ajisai")]
    Ajisai,
    #[serde(rename = "kuro")]
    Kuro,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGraphParam {
    pub name: String,
    pub unit: String,
    pub color: GraphColor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphDefinitions {
    pub graphs: Vec<Graph>,
}

pub(crate) fn create<T>(auth: &Authentication, param: &Graph) -> Result<(), Error> where T: HttpClient {
    let uri = endpoint::graphs(&auth.username);
    let body = serde_json::to_string(param)?;
    let context = RequestContext::new(
        &uri,
        MethodType::Post,
        Some(&body),
        Some(&auth.token),
    );

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

pub(crate) fn update<T>(auth: &Authentication, graph_id: &str, param: &UpdateGraphParam) -> Result<(), Error> where T: HttpClient {
    let uri = endpoint::graph(&auth.username, graph_id);
    let body = serde_json::to_string(param)?;
    let context = RequestContext::new(
        &uri,
        MethodType::Put,
        Some(&body),
        Some(&auth.token),
    );

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

pub(crate) fn delete<T>(auth: &Authentication, graph_id: &str) -> Result<(), Error> where T: HttpClient {
    let uri = endpoint::graph(&auth.username, graph_id);
    let context = RequestContext::new(
        &uri,
        MethodType::Delete,
        None,
        Some(&auth.token),
    );

    let body = T::do_request(&context)?;
    response::build_result(&body)
}

pub(crate) fn get_all<T>(auth: &Authentication) -> Result<Vec<Graph>, Error> where T: HttpClient {
    let uri = endpoint::graphs(&auth.username);
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

    let res: GraphDefinitions = serde_json::from_str(&body)?;
    Ok(res.graphs)
}

pub(crate) fn get_graph_svg<T>(auth: &Authentication, graph_id: &str, date: Option<&str>) -> Result<String, Error> where T: HttpClient {
    let uri = endpoint::graph_svg(&auth.username, graph_id, date);
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

    Ok(body)
}


#[cfg(test)]
mod graph_test {
    use super::*;

    #[test]
    fn graph_serialize_test() {
        let param = Graph {
            id: String::from("testid"),
            name: String::from("testname"),
            unit: String::from("testunit"),
            graph_type: GraphType::Int,
            color: GraphColor::Kuro,
        };

        let res = serde_json::to_string(&param);
        if let Ok(v) = res {
            assert_eq!(v, r#"{"id":"testid","name":"testname","unit":"testunit","type":"int","color":"kuro"}"#);
        } else {
            panic!("failed serialize params");
        };
    }

    #[test]
    fn graphs_deserialize_test() {
        let json = r#"{"graphs":[{"id":"testid","name":"testname","unit":"testunit","type":"int","color":"kuro"}]}"#;
        let res: GraphDefinitions = serde_json::from_str(json).unwrap();
        assert_eq!(res.graphs.len(), 1);
        assert_eq!(&res.graphs[0].id, "testid");
        assert_eq!(&res.graphs[0].name, "testname");
        assert_eq!(&res.graphs[0].unit, "testunit");
        assert_eq!(&res.graphs[0].graph_type, &GraphType::Int);
        assert_eq!(&res.graphs[0].color, &GraphColor::Kuro);
    }
}