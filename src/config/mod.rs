
use serde::{Serialize,Deserialize};
use serde_json::Value;
use surrealdb::opt::auth::{Credentials,Jwt};
pub mod parser;
mod auth;

/// The name of default configuration file
/// Used when the specified configuration file location is not passed in
/// Speculate by constructing the default configuration file address
/// based on the current project directory and the file name
const DEFAULT_CONFIG_NAME:&str="surrealdb.config.json";

/// certified bridge
/// All credential types capable of login authentication should implement this trait
pub trait AuthBridger<'a,Action>{
    type AuthType;
    ///get a low-level instance, and the return value is the actual type
    fn to_lower_cast(&'a self)->Self::AuthType
    where
        Self::AuthType:Credentials<Action,Jwt>;

    fn keys()->Vec<&'a str>;

    ///Convert to low-level instance and doesn't cumstom itself
    fn to_lower(&'a self)->impl Credentials<Action,Jwt>
}