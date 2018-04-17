#![allow(dead_code, unused_variables)]

mod diesel_api;

use transaction::Transaction;
use std::collections::HashMap;

pub fn list() -> Vec<Box<Template>> {
    vec![
        Box::new(diesel_api::DieselApiTemplate)
    ]
}

pub trait Template {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn parameters(&self) -> Vec<TemplateParameter>;
    fn execute(&self, transaction: &mut Transaction, parameters: HashMap<String, String>) -> Result<(), String>;
}

pub struct TemplateParameter {
    pub name: String,
    pub description: String,
    pub validations: Vec<Box<ParameterValidator>>,
}

impl TemplateParameter {
    pub fn new<T: Into<String>, U: Into<String>>(name: T, description: U) -> TemplateParameter {
        TemplateParameter {
            name: name.into(),
            description: description.into(),
            validations: Vec::new()
        }
    }

    pub fn with_validator<T: ParameterValidator + 'static>(mut self, validator: T) -> Self {
        self.validations.push(Box::new(validator));
        self
    }
}

pub trait ParameterValidator {
    fn validate(&self, value: &str, current_values: &HashMap<String, String>) -> Result<(), String>;
}

pub struct NumberValidator;

impl ParameterValidator for NumberValidator {
    fn validate(&self, value: &str, current_values: &HashMap<String, String>) -> Result<(), String> {
        if let Err(e) = value.parse::<u32>() {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }
}
