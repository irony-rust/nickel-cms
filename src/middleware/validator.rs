pub use rustc_serialize::json::{self, Json, ToJson};
pub use rustc_serialize::Decodable;
use params::{Value, FromValue};
use super::render::BaseDataMap;
use std::collections::BTreeMap;
use std::string::String;

//
#[derive(RustcDecodable, Debug)]
pub struct Validator<T> {
    pub vtype: String,
    pub requiered: Option<bool>,
    pub empty: Option<bool>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub dafault: Option<T>,
    errors: Option<ErrorValidator>,
}

#[derive(Debug)]
pub struct ValidateResult(BaseDataMap, ErrorValidator);
#[derive(Debug)]
pub struct ValidateResults(pub Vec<ValidateResult>);
pub type ErrorsResult = Option<Vec<ErrorValidator>>;

impl ValidateResults {
    /// Get Validation Errors result
    pub fn get_errors(&self) -> ErrorsResult {
        let &ValidateResults(ref results) = self;
        let mut errors = vec!();
        for &ValidateResult(_, ref err) in results {
            if err.errors_count.is_some() {
                errors.push(err.to_owned());
            }
        }
        if errors.len() > 0 { Some(errors) } else { None }
    }

    /// Get Validation Values result
    pub fn get_values(&self) -> BaseDataMap {
        let &ValidateResults(ref results) = self;
        let mut values: BaseDataMap = BTreeMap::new();
        for &ValidateResult(ref val, _) in results {
            values.append(&mut val.clone());
        }
        values
    }
}

impl<T: FromValue + ToJson> Validator<T> {
    pub fn new(validator_rules: BaseDataMap) -> Validator<String> {
        let json_obj: Json = Json::Object(validator_rules);
        let json_str: String = json_obj.to_string();
        json::decode(&json_str).unwrap()
    }

    pub fn validate(&mut self, field: String, value: Option<&Value>) -> ValidateResult {
        // Init Errors
        self.errors = Some(ErrorValidator::new(&field));

        // Invoke validators
        self.requiered(value);

        let json_value: Json = match self.type_cast(value) {
            Some(ref json_value) => json_value.to_owned(),
            None => {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field requiered: {}", error.field);
                    error.add(msg);
                }
                "".to_json()
            }
        };

        let mut err = ErrorValidator::new(&field);
        if let Some(ref err_results) = self.errors {
            err = err_results.to_owned();
        }
        ValidateResult(btreemap! {
            field.to_owned() => json_value
        }, err)
    }

    /// Requered validator
    fn requiered(&mut self, value: Option<&Value>) {
        if self.requiered.is_some() {
            if value.is_none() {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field requiered: {}", error.field);
                    error.add(msg);
                }
            }
        }
    }

    fn type_cast(&self, value: Option<&Value>) -> Option<Json> {
        let mut val: Value;
        if let Some(name) = value {
            val = name.to_owned();
        } else {
            return None
        }
        match &self.vtype.as_ref() as &str {
            "bool" | "boolean" => {
                if let Some(val) = <bool as FromValue>::from_value(&val) {
                    Some(Json::Boolean(val))
                } else {
                    return None;
                }
            },
            "string" | "str" => {
                if let Some(val) = <String as FromValue>::from_value(&val) {
                    Some(Json::String(val))
                } else {
                    return None;
                }
//                let val = <String as FromValue>::from_value(&val).unwrap();
//                Some(Json::String(val))
            },
            _ => None,
        }

//        let ok_value: Value = match self.vtype.as_ref() {
//            "u8" | "u16" | "u32" | "u64" => {
//                if let Some(val) = <T as FromValue>::from_value(&val) {
//                    Value::U64(val)
//                } else {
//                    return None;
//                }
//            },
//            "u16" => <u16 as FromValue>::from_value(&val),
//            "u32" => <u32 as FromValue>::from_value(&val),
//            "u64" => <u64 as FromValue>::from_value(&val),
//            "usize" => <usize as FromValue>::from_value(&val),
//            "i8"  => <u8  as FromValue>::from_value(&val),
//            "i16" => <u16 as FromValue>::from_value(&val),
//            "i32" => <u32 as FromValue>::from_value(&val),
//            "i64" => <u64 as FromValue>::from_value(&val),
//            "isize" => <usize as FromValue>::from_value(&val),
//            "f32" => <f32 as FromValue>::from_value(&val),
//            "f64" => <f64 as FromValue>::from_value(&val),
//            "string" => <String as FromValue>::from_value(&val),
//            "bool" => {
//                if let Some(val) = <bool as FromValue>::from_value(&val) {
//                    Value::Boolean(val)
//                } else {
//                    return None;
//                }
//            },
//            _ => return None,
//        };
//        println!("===| {:?}", ok_value);

//        if self.vtype == "string" {
//            println!("===| {:?}", self.vtype);
//            let _n1 = Value::from_value(&val);
//            println!("===| {:?}", _n1);
//            if let Some(&Value::String(ref name)) = value {
//                Value::Null => f.write_str("null"),
//                Value::Boolean(value) => value.fmt(f),
//                Value::I64(value) => value.fmt(f),
//                Value::U64(value) => value.fmt(f),
//                Value::F64(value) => value.fmt(f),
//                Value::String( ref value) => value.fmt(f),
//                Value::File( ref value) => value.fmt(f),
//                Value::Array( ref value) => value.fmt(f),
//                Value::Map( ref value) => value.fmt(f),
//            }
//        }
//        None
    }
}

/// Validator Errors
#[derive(RustcDecodable, Debug, Clone)]
pub struct ErrorValidator {
    pub errors: Vec<String>,
    pub errors_count: Option<u32>,
    pub field : String,
}

/// Validator Errors methods
impl ErrorValidator {
    /// Init error
    fn new(field: &String) -> Self {
        ErrorValidator {
            field: field.to_owned(),
            errors: vec!(),
            errors_count: None,
        }
    }

    /// Add error
    fn add(&mut self, error: String) {
        if self.errors_count.is_none() {
            self.errors = vec!(error);
            self.errors_count = Some(1);
        } else {
            if let Some(count) = self.errors_count {
                self.errors.push(error);
                self.errors_count = Some(count + 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new_test() {
        let val_req = Validator::<String>::new(btreemap! {
            "requiered".to_string() => true.to_json(),
            "vtype".to_string() => "bool".to_json(),
        });
        assert_eq!(val_req.requiered, Some(true));
        assert!(val_req.errors.is_none());
    }
}
