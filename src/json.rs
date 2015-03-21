use std::error::FromError;
use std::fmt;

use serialize::json::{BuilderError, Json, Object};

/// Errors that can occur when trying to parse a template.
pub enum TemplateError<'a> {
    JsonError(BuilderError),
    // field
    MissingField(&'a str),
    // field, type
    InvalidType(&'a str, &'a str),
}

impl<'a> FromError<BuilderError> for TemplateError<'a> {

    fn from_error(err: BuilderError) -> TemplateError<'a> {
        TemplateError::JsonError(err)
    }

}

impl<'a> fmt::Debug for TemplateError<'a> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemplateError::JsonError(ref err) => err.fmt(f),
            TemplateError::MissingField(field) => write!(f, "Field {} is required but missing.", field),
            TemplateError::InvalidType(field, typ) => write!(f, "Fied {} should be a/an {}.", field, typ),
        }
    }

}

/// Unwrap a field from a JSON object.
/// Return an error if the field is not found.
macro_rules! json_field {
    ( $j:expr, $n:expr ) => {
        match $j.find($n) {
            None => return Err(TemplateError::MissingField($n)),
            Some(field) => field,
        }
    }
}

/// Unwrap a string from a JSON object.
macro_rules! json_string {
    ( $j:expr, $n:expr ) => {
        match json_field!($j, $n).as_string() {
            None => return Err(TemplateError::InvalidType($n, "string")),
            Some(val) => val,
        }
    }
}

/// Unwrap an object from a JSON object.
macro_rules! json_object {
    ( $j:expr, $n:expr ) => {
        match json_field!($j, $n).as_object() {
            None => return Err(TemplateError::InvalidType($n, "object")),
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
                    None => return Err(TemplateError::InvalidType($n, "object")),
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

impl<'a> ResponseTemplate<'a> {

    pub fn new<'b>(json: &'b Json) -> Result<ResponseTemplate<'b>, TemplateError> {
        if !json_is_field_object(json, "result") && !json_is_field_object(json, "error") {
            return Err(TemplateError::MissingField("result/error"));
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

/// Represents a Notification
pub struct NotificationTemplate<'a> {
    pub kind: &'a str,
    pub params: &'a Object,
}

impl<'a> NotificationTemplate<'a> {

    pub fn new<'b>(json: &'b Json) -> Result<NotificationTemplate<'b>, TemplateError> {
        // Ensure id is present and null.
        match json.find("id") {
            None => return Err(TemplateError::MissingField("id")),
            Some(id) => {
                if !id.is_null() {
                    return Err(TemplateError::InvalidType("id", "null"));
                }
            }
        }

        Ok(NotificationTemplate {
            kind: json_string!(json, "kind"),
            params: json_object!(json, "params"),
        })
    }

}

#[test]
fn valid_req_template() {
    let json = Json::from_str("{\"id\":\"1\",\"kind\":\"irc/something\",\"params\":{}}").unwrap();
    let res = RequestTemplate::new(&json);
    assert!(res.is_ok());
}

#[test]
fn bad_req_template_no_id() {
    let json = Json::from_str("{\"kind\":\"irc/something\",\"params\":{}}").unwrap();
    let res = RequestTemplate::new(&json);
    assert!(res.is_err());
}

#[test]
fn bad_req_template_no_kind() {
    let json = Json::from_str("{\"id\":\"1\",\"params\":{}}").unwrap();
    let res = RequestTemplate::new(&json);
    assert!(res.is_err());
}

#[test]
fn bad_req_template_no_params() {
    let json = Json::from_str("{\"id\":\"1\",\"kind\":\"irc/something\"}").unwrap();
    let res = RequestTemplate::new(&json);
    assert!(res.is_err());
}

#[test]
fn valid_res_template_result() {
    let json = Json::from_str("{\"id\":\"1\",\"kind\":\"irc/something\",\"result\":{}}").unwrap();
    let res = ResponseTemplate::new(&json);
    assert!(res.is_ok());
}

#[test]
fn valid_res_template_error() {
    let json = Json::from_str("{\"id\":\"1\",\"kind\":\"irc/something\",\"error\":{}}").unwrap();
    let res = ResponseTemplate::new(&json);
    assert!(res.is_ok());
}

#[test]
fn bad_res_template_no_id() {
    let json = Json::from_str("{\"kind\":\"irc/something\",\"error\":{}}").unwrap();
    let res = ResponseTemplate::new(&json);
    assert!(res.is_err());
}

#[test]
fn bad_res_template_no_kind() {
    let json = Json::from_str("{\"id\":\"1\",\"result\":{}}").unwrap();
    let res = ResponseTemplate::new(&json);
    assert!(res.is_err());
}

#[test]
fn bad_res_template_no_result_or_error() {
    let json = Json::from_str("{\"id\":\"1\",\"kind\":\"irc/something\"}").unwrap();
    let res = ResponseTemplate::new(&json);
    assert!(res.is_err());
}

#[test]
fn valid_notif_template() {
    let json = Json::from_str("{\"id\":null,\"kind\":\"irc/something\",\"params\":{}}").unwrap();
    let res = NotificationTemplate::new(&json);
    assert!(res.is_ok());
}

#[test]
fn bad_notif_template_no_id() {
    let json = Json::from_str("{\"kind\":\"irc/something\",\"params\":{}}").unwrap();
    let res = NotificationTemplate::new(&json);
    assert!(res.is_err());
}

#[test]
fn bad_notif_template_id_not_null() {
    let json = Json::from_str("{\"id\":1,\"kind\":\"irc/something\",\"params\":{}}").unwrap();
    let res = NotificationTemplate::new(&json);
    assert!(res.is_err());
}

#[test]
fn bad_notif_template_no_kind() {
    let json = Json::from_str("{\"id\":null,\"params\":{}}").unwrap();
    let res = NotificationTemplate::new(&json);
    assert!(res.is_err());
}

#[test]
fn bad_notif_template_no_params() {
    let json = Json::from_str("{\"id\":null,\"kind\":\"irc/something\"}").unwrap();
    let res = NotificationTemplate::new(&json);
    assert!(res.is_err());
}
