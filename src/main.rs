extern crate bincode;

mod templates;
mod transaction;

use std::env::args;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut iter = args();
    let _self_name = match iter.next() {
        Some(n) => n,
        None => {
            // What?
            return;
        }
    };
    let name = match iter.next()  {
        Some(n) => n,
        None => {
            print_help();
            return;
        }
    };

    let templates = templates::list();

    let template = match templates.into_iter().find(|i| i.name() == name) {
        Some(n) => n,
        None => {
            print_help();
            println!("Template {:?} not found", name);
            return;
        }
    };

    println!("{:20}{}", template.name(), template.description());
    println!();
    let parameters = template.parameters();
    let parameters_length = parameters.len();
    let mut parameter_values: HashMap<String, String> = HashMap::with_capacity(parameters.len());

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    'parameter_loop: for (index, parameter) in parameters.into_iter().enumerate() {
        println!("{:20}{}", format!("{}/{}: {}", index, parameters_length, parameter.name), parameter.description);

        'input_loop: loop {
            if let Some(Ok(line)) = iterator.next() {
                for validator in &parameter.validations {
                    if let Err(e) = validator.validate(&line, &parameter_values) {
                        println!("{}", e);
                        continue 'input_loop;
                    }
                }
                parameter_values.insert(parameter.name, line);
                break 'input_loop;
            }
        }
    }

    let mut transaction = transaction::Transaction::default();
    if let Err(e) = template.execute(&mut transaction, parameter_values) {
        println!("{}", e);
        return;
    }

    println!("Done!");
    println!("Transactions: {:#?}", transaction);
}

fn print_help() {
    println!("Usage: cargo template <template_name>");
    println!("The remaining arguments will be asked in the wizard");
    println!("Available templates:");
    for template in templates::list() {
        println!("{:20}{}", template.name(), template.description());
    }
}
