pub mod cli;
pub(crate) mod config;
mod req;
mod utils;

pub use config::{DiffConfig, DiffProfile, ResponseProfile};
pub use req::RequestProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}
