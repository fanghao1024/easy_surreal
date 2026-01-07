use super::{SurrealConfig,DEFAULT_CONFIG_NAME};
use serde_json::Value;

use std::{
    env::current_dir,
    fs::{canonicalize,read_to_string},
    path::{Path,PathBuf},
};

///# Parser enumeration
/// It can incorporate various parsers, such as JSON parser, TOML parser ,YAML parser, etc<br>
/// Now only have implemented the JSON parser<br>
/// For external entitles, only the Parsers enumeration should be used to select a parser for parsing<br>
pub enum Parsers{
    Json,
}

impl Parsers{
    pub fn parse(&self,path:Option<&str>)->Value{
        match self{
            Parsers::Json => JsonParser::parse(path),
            _=>panic!("Invalid Parser"),
        }
    }

    pub fn json()->JsonParser{
        JsonParser
    }
    /// ## Parse into the format of SurrealConfig <br>
    /// Use SurrealConfig directly within the framework <br>
    /// Utilize SurrealConfig to obtain specific configuration information for use
    pub fn parse_to_config(&self,path:Option<&str>)->SurrealConfig{
        let config:SurrealConfig=self.parse(path).into();
        config
    }

}


/// JSON Parser <br>
/// Used to parse configuration files in JSON format <br>
/// Convert the JSON file into a unified serde_json::Value <br>
pub struct JsonParser;

impl JsonParser{
    pub fn parse<P>(path:Option<P>)->Value
    where
        P:AsRef<Path>
    {
        let path:PathBuf=match path{
            Some(p)=>canonicalize(p).unwrap(),
            None =>{
                let mut current_dir=current_dir().unwrap();
                let _=current_dir.push(DEFAULT_CONFIG_NAME);
                current_dir
            }
        };
        let config_str=read_to_string(path.as_path()).unwrap_or(String::new());
        let res:Value=serde_json::from_str(&config_str).unwrap();
        return res;
    }
}

#[cfg(test)]
mod parse_test {
    use std::path::Path;

    use serde_json::Value;

    use super::{JsonParser,Parsers};

    #[test]
    fn test_json_str_match(){
        let json_str=r#"
        {
            "endpoint":"127.0.0.1",
            "port":10086,
            "auth":{
                "user":"root",
                "pass":"root",
            }
        }
        "#;
        let json_value1:Value=serde_json::from_str(json_str).unwrap();
        let json_value2=JsonParser::parse(Some(Path::new("./surrealdb.config.json")));
        assert_eq!(json_value1,json_value2)
    }

}