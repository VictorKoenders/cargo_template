use super::{Template, TemplateParameter};
use transaction::Transaction;
use std::collections::HashMap;

pub struct DieselApiTemplate;

impl Template for DieselApiTemplate {
    fn name(&self) -> &'static str {
        "diesel_api"
    }
    fn description(&self) -> &'static str {
        "Generates a rocket API endpoint for a given diesel table"
    }
    fn parameters(&self) -> Vec<TemplateParameter> {
        vec![
            TemplateParameter::new("table", "Table name")
                .with_validator(TableExistsValidator)
        ]
    }

    fn execute(&self, transaction: &mut Transaction, parameters: HashMap<String, String>) -> Result<(), String> {
        Err(String::from("Not implemented"))
    }
}

pub struct TableExistsValidator;

impl super::ParameterValidator for TableExistsValidator {
    fn validate(&self, value: &str, current_values: &HashMap<String, String>) -> Result<(), String> {
        Err(String::from("Could not validate parameter, not implemented"))
    }
}
