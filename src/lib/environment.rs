use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
};

use crate::data_type::DataType;

pub struct Environment {
    pub variables: HashMap<String, Box<dyn Any>>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: Box<dyn Any>) -> Result<(), &'static str> {
        self.variables.insert(name, value);
        return Ok(());
    }

    pub fn assign(&mut self, name: String, value: Box<dyn Any>) -> Result<(), &'static str> {
        if let Entry::Occupied(mut variables) = self.variables.entry(name) {
            variables.insert(value);
        } else {
            return Err("Undefined variable '{name}'.");
        }
        return Ok(());
    }

    pub fn get(&self, name: &str) -> Result<&Box<dyn Any>, &'static str> {
        if let Some(value) = self.variables.get(name) {
            return Ok(value);
        } else {
            return Err("Undefined variable '{name}'.");
        }
    }

    pub fn data_type(&self, name: &str) -> Result<DataType, &'static str> {
        let object = self.variables.get(name);
        if object.is_none() {
            return Err("Undefined variable '{name}'.");
        }
        return Ok(DataType::box_any_to_data_type(object.unwrap()).unwrap());
    }
}
