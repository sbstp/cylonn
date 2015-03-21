use serialize::json::{BuilderError, Json, Object};

/// Errors that can occur when trying to parse a template.
pub enum TemplateError {
    JsonError(BuilderError),
    MissingField(String),
    InvalidType(String),
}

// Implement FromError for json::BuilderError.
from_error!(TemplateError, BuilderError, TemplateError::JsonError);

/// Unwrap a field from a JSON object.
/// Return an error if the field is not found.
macro_rules! json_field {
    ( $j:expr, $n:expr ) => {
        match $j.find($n) {
            None => return Err(TemplateError::MissingField($n.to_string())),
            Some(field) => field,
        }
    }
}

/// Unwrap a string from a JSON object.
macro_rules! json_string {
    ( $j:expr, $n:expr ) => {
        match json_field!($j, $n).as_string() {
            None => return Err(TemplateError::InvalidType($n.to_string())),
            Some(val) => val,
        }
    }
}

/// Unwrap an object from a JSON object.
macro_rules! json_object {
    ( $j:expr, $n:expr ) => {
        match json_field!($j, $n).as_object() {
            None => return Err(TemplateError::InvalidType($n.to_string())),
            Some(val) => val,
        }
    }
}

/// Represents a Request.
pub struct RequestTemplate<'a> {
    pub id: &'a str,
    pub kind: &'a str,
    pub params: &'a Object,
}

impl<'a> RequestTemplate<'a> {

    pub fn new<'b>(json: &'b Json) -> Result<RequestTemplate<'b>, TemplateError> {
        Ok(RequestTemplate {
            id: json_string!(json, "id"),
            kind: json_string!(json, "kind"),
            params: json_object!(json, "params"),
        })
    }

}

/// None if the field does not exist. If it exists, try to unwrap as object.
/// Return if is it not an object.
macro_rules! json_opt_object {
    ( $j:expr, $n:expr ) => {
        match $j.find($n) {
            None => None,
            Some(val) => {
                match val.as_object() {
                    None => return Err(TemplateError::InvalidType($n.to_string())),
                    Some(val) => Some(val),
                }
            }
        }
    }
}

/// Represents a Response
pub struct ResponseTemplate<'a> {
    pub id: &'a str,
    pub kind: &'a str,
    pub error: Option<&'a Object>,
    pub result: Option<&'a Object>,
}

impl <'a> ResponseTemplate<'a> {

    pub fn new<'b>(json: &'b Json) -> Result<ResponseTemplate<'b>, TemplateError> {
        if !json_is_field_object(json, "result") && !json_is_field_object(json, "error") {
            return Err(TemplateError::MissingField("result/error".to_string()));
        }

        Ok(ResponseTemplate {
            id: json_string!(json, "id"),
            kind: json_string!(json, "kind"),
            error: json_opt_object!(json, "error"),
            result: json_opt_object!(json, "result"),
        })
    }

}

/// Test if a field exists and is an object.
fn json_is_field_object(json: &Json, field: &str) -> bool {
    match json.find(field) {
        None => false,
        Some(ref val) => val.is_object(),
    }
}

#[cfg(test)]
mod tests {
    use serialize::json::Json;
    use super::{RequestTemplate, ResponseTemplate};

    #[test]
    fn valid_req_template() {
        let json = Json::from_str("{\"id\":\"1\",\"kind\":\"irc/something\",\"params\":{}}").unwrap();
        let res = RequestTemplate::new(&json);
        assert!(res.is_ok());
        let tpl = res.ok().unwrap();
        assert_eq!(tpl.id, "1");
    }

    #[test]
    fn bad_req_template() {
        let json = Json::from_str("{\"pid\":\"1\",\"kong\":\"irc/something\",\"parents\":{}}").unwrap();
        let res = RequestTemplate::new(&json);
        assert!(res.is_err());
    }

}
