
mod parser;

pub enum JSON {
    Null,
    Bool(bool),
    Number(),
    String(),
    Object(),
    Array(Vec<JSON>)
};


fn parse_bool() -> Result<JSON, JsonError> {

}

fn parse_number() -> Result<JSON, JsonError> {
    
}

fn parse_string() -> Result<JSON, JsonError> {
    
}

fn parse_object() -> Result<JSON, JsonError> {
    
}

fn parse_array() -> Result<JSON, JsonError> {
    
}


pub fn parse(input: &str) -> Result<JSON, JsonError> {

}



