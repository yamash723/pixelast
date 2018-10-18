#![doc(html_root_url = "https://docs.rs/pixelast/0.1.0")]

//! This library is Pixela client for Rust.
//! 
//! # Example
//! 
//! ```rust,ignore
//! use pixelast::{PixelaClient, ConsentAnswer, GraphType, GraphColor};
//! 
//! fn main() {
//!     let res = PixelaClient::create_new_user(
//!         "username",
//!         "usertoken",
//!         ConsentAnswer::Yes,
//!         ConsentAnswer::Yes
//!     );
//! 
//!     match res {
//!         Ok(()) => println!("create new user."),
//!         Err(v) => panic!("create new user failed. {}", v),
//!     }
//! 
//!     let client = PixelaClient::new("username", "usertoken");
//!     client.create_graph("graphid", "graphname", "cal", GraphType::Int, GraphColor::Shibafu).unwrap();
//!     client.update_graph("graphid", "graphname", "cal", GraphColor::Shibafu).unwrap();
//! 
//!     client.record_pixel("graphid", "20181016", "10").unwrap();
//!     client.record_pixel("graphid", "20181017", "10").unwrap();
//!     client.record_pixel("graphid", "20181018", "10").unwrap();
//! 
//!     client.update_pixel("graphid", "20181018", "20").unwrap();
//!     client.delete_pixel("graphid", "20181016").unwrap();
//! 
//!     let pixel = client.get_pixel("graphid", "20181018").unwrap();
//!     println!("{:?}", pixel);
//! 
//!     client.increment("graphid").unwrap();
//!     client.decrement("graphid").unwrap();
//! 
//!     let graphs = client.get_graphs().unwrap();
//!     println!("{:?}", graphs);
//! 
//!     let svg = client.get_graph_svg("graphid", Some("20181020")).unwrap();
//!     println!("{}", svg);
//! 
//!     client.delete_graph("graphid").unwrap();
//!     client.delete_user().unwrap();
//! }
//! ```

extern crate reqwest;
extern crate regex;
extern crate serde;
extern crate serde_json;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;

use failure::Error;

mod response;
mod endpoint;
mod http_client;
mod error;
mod graph;
mod pixel;
mod user;

pub use self::user::ConsentAnswer;
pub use self::graph::{Graph, GraphType, GraphColor};
pub use self::pixel::Pixel;
pub use self::error::PixelaClientError;

use self::user::CreateUserParam;
use self::graph::UpdateGraphParam;
use self::http_client::TinyHttpClient;

/// A `PixelaClient` to request to Pixela with.
pub struct PixelaClient {
    auth: Authentication,
}

pub(crate) struct Authentication {
    username: String,
    token: String,
}

impl PixelaClient {
    /// Constructs a new `PixelaClient`.
    ///
    /// This method does not verify authentication.
    pub fn new(username: &str, token: &str) -> Self {
        Self {
            auth: Authentication {
                username: username.to_owned(),
                token: token.to_owned(),
            }
        }
    }

    /// Create a new Pixela user.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn create_new_user(username: &str, token: &str, agree_terms_of_service: ConsentAnswer, not_minor: ConsentAnswer) -> Result<(), Error> {
        let param = CreateUserParam {
            username: username.to_owned(),
            token: token.to_owned(),
            agree_terms_of_service,
            not_minor
       };

        user::create::<TinyHttpClient>(&param)
    }

    /// Updates the authentication token for the specified user.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn update_user_token(&self, new_token: &str) -> Result<(), Error> {
        user::update::<TinyHttpClient>(&self.auth, new_token)
    }

    /// Deletes the specified registered user.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn delete_user(&self) -> Result<(), Error> {
        user::delete::<TinyHttpClient>(&self.auth)
    }

    /// Create a new pixelation graph definition.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn create_graph(&self, id: &str, name: &str, unit: &str, graph_type: GraphType, color: GraphColor) -> Result<(), Error> {
        let param = Graph {
            id: id.to_owned(),
            name: name.to_owned(),
            unit: unit.to_owned(),
            graph_type,
            color,
        };

        graph::create::<TinyHttpClient>(&self.auth, &param)
    }

    /// Get all predefined pixelation graph definitions.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn get_graphs(&self) -> Result<Vec<Graph>, Error> {
        graph::get_all::<TinyHttpClient>(&self.auth)
    }

    /// Based on the registered information, express the graph in SVG format diagram.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn get_graph_svg(&self, graph_id: &str, date: Option<&str>) -> Result<String, Error> {
        graph::get_graph_svg::<TinyHttpClient>(&self.auth, graph_id, date)
    }

    /// Update predefined pixelation graph definitions. The items that can be updated are limited as compared with the pixelation graph definition creation.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn update_graph(&self, graph_id: &str, name: &str, unit: &str, color: GraphColor) -> Result<(), Error> {
        let param = UpdateGraphParam {
            name: name.to_owned(),
            unit: unit.to_owned(),
            color,
        };

        graph::update::<TinyHttpClient>(&self.auth, graph_id, &param)
    }

    /// Delete the predefined pixelation graph definition.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn delete_graph(&self, graph_id: &str) -> Result<(), Error> {
        graph::delete::<TinyHttpClient>(&self.auth, graph_id)
    }

    /// It records the quantity of the specified date as a "Pixel".
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn record_pixel(&self, graph_id: &str, date: &str, quantity: &str) -> Result<(), Error> {
        let param = Pixel {
            date: date.to_owned(),
            quantity: quantity.to_owned(),
        };

        pixel::create::<TinyHttpClient>(&self.auth, graph_id, &param)
    }

    /// Get registered quantity as "Pixel".
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn get_pixel(&self, graph_id: &str, date: &str) -> Result<Pixel, Error> {
        pixel::get::<TinyHttpClient>(&self.auth, graph_id, date)
    }

    /// Update the quantity already registered as a "Pixel".
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn update_pixel(&self, graph_id: &str, date: &str, quantity: &str) -> Result<(), Error> {
        let param = Pixel {
            date: date.to_owned(),
            quantity: quantity.to_owned(),
        };

        pixel::update::<TinyHttpClient>(&self.auth, graph_id, &param)
    }

    /// Delete the registered "Pixel".
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn delete_pixel(&self, graph_id: &str, date: &str) -> Result<(), Error> {
        pixel::delete::<TinyHttpClient>(&self.auth, graph_id, date)
    }

    /// Increment quantity "Pixel" of the day (UTC).
    /// If the graph type is int then 1 added, and for float then 0.01 added.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn increment(&self, graph_id: &str) -> Result<(), Error> {
        pixel::increment::<TinyHttpClient>(&self.auth, graph_id)
    }

    /// Decrement quantity "Pixel" of the day (UTC).
    /// If the graph type is int then -1 added, and for float then -0.01 added.
    ///
    /// # Errors
    ///
    /// This method fails when request not success in Pixela.
    pub fn decrement(&self, graph_id: &str) -> Result<(), Error> {
        pixel::decrement::<TinyHttpClient>(&self.auth, graph_id)
    }
}
