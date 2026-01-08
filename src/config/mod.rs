
use serde::{Serialize,Deserialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use surrealdb::opt::auth::{Credentials,Jwt};
use self::auth::AuthCredentials;
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
    fn to_lower(&'a self)->impl Credentials<Action,Jwt>;
}

//SurrealDB的配置
#[derive(Debug,Serialize,Clone)]
pub struct SurrealConfig {
    //Startup url
    endpoint:String,
    //Startup point
    port:u16,
    //Login credential data
    auth:Value,
}

// convert serde_json::Value to SurrealConfig
impl From<Value> for SurrealConfig {
    fn from(value: Value) -> Self {
        let endpoint=value.get("endpoint").unwrap().as_str().unwrap().to_string();
        let port=value.get("port").unwrap().as_u64().unwrap() as u16;
        let auth=value.get("auth").unwrap().clone();
        Self{
            endpoint,
            port,
            auth,
        }
    }
}

impl SurrealConfig{
    ///Retrieves login credential data
    /// All credentials can actually be converted
    /// In fact, users may have no idea what type of login crediential they are
    /// @return AuthCredientials
    pub fn get_auth<P>(&self)->AuthCredientials<P>
    where
        P:Serialize+DeserializeOwned
    {
        let res:AuthCredentials<P>=self.auth.clone().into();
        res
    }

    /// Get the configured SurrealDB address
    pub fn get_endpoint(&self)->&str{
        &self.endpoint
    }

    /// Get the configured SurrealDB port
    pub fn get_port(&self)->u16{
        self.port
    }

    /// Get URL, the actural format is {{address}}:{{port}}
    pub fn url(&self)->String{
        format!("{}:{}",self.endpoint,self.port)
    }
}

#[cfg(test)]
mod test_config{
    use serde_json::Value;
    use crate::config::auth::AuthCredentials;
    use super::{parser::Parsers,SurrealConfig};

    #[test]
    fn test_parser_config(){
        let json=Parsers::Json.parse(None);
        let config:SurrealConfig=json.into();
        dbg!(&config);
        let auth_credential:AuthCredentials<Value>=config.get_auth();
        dbg!(&auth_credential);
    }
}