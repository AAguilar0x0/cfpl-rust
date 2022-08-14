use std::any::Any;

use crate::{token::Token, token_type::TokenType};

#[derive(Clone, PartialEq, Debug)]
pub enum DataType {
    INT,
    FLOAT,
    CHAR,
    BOOL,
    STR,
}

impl DataType {
    pub fn stringify_primitives(object: &Box<dyn Any>) -> Result<String, String> {
        let string = if let Some(output) = object.downcast_ref::<i32>() {
            output.to_string()
        } else if let Some(output) = object.downcast_ref::<f64>() {
            output.to_string()
        } else if let Some(output) = object.downcast_ref::<char>() {
            output.to_string()
        } else if let Some(output) = object.downcast_ref::<bool>() {
            output.to_string().to_uppercase()
        } else if let Some(output) = object.downcast_ref::<String>() {
            output.clone()
        } else {
            return Err("Invalid data type.".to_owned());
        };
        return Ok(string);
    }

    pub fn box_any_to_data_type(object: &Box<dyn Any>) -> Option<DataType> {
        let mut object = object;
        while let Some(value) = (*object).downcast_ref::<Box<dyn Any>>() {
            object = value;
        }
        return if (*object).downcast_ref::<i32>().is_some() {
            Some(DataType::INT)
        } else if (*object).downcast_ref::<f64>().is_some() {
            Some(DataType::FLOAT)
        } else if (*object).downcast_ref::<char>().is_some() {
            Some(DataType::CHAR)
        } else if (*object).downcast_ref::<bool>().is_some() {
            Some(DataType::BOOL)
        } else if (*object).downcast_ref::<String>().is_some() {
            Some(DataType::STR)
        } else {
            None
        };
    }

    pub fn str_to_data_type(str: &str, data_type: &TokenType) -> Option<Box<dyn Any>> {
        return match data_type {
            TokenType::LitBool => {
                Some(Box::new(str.trim().to_lowercase().parse::<bool>().unwrap()))
            }
            TokenType::LitChar => Some(Box::new(str.trim().parse::<char>().unwrap())),
            TokenType::LitFloat => Some(Box::new(str.trim().parse::<f64>().unwrap())),
            TokenType::LitInt => Some(Box::new(str.trim().parse::<i32>().unwrap())),
            TokenType::LitStr => Some(Box::new(str.to_owned())),
            _ => None,
        };
    }

    pub fn clone_ref_any(object: &Box<dyn Any>) -> Option<Box<dyn Any>> {
        let data_type = DataType::box_any_to_data_type(object)?;
        let mut object = object;
        while let Some(value) = (*object).downcast_ref::<Box<dyn Any>>() {
            object = value;
        }
        let value: Box<dyn Any> = match data_type {
            DataType::INT => Box::new(*(*object).downcast_ref::<i32>().unwrap()),
            DataType::FLOAT => Box::new(*(*object).downcast_ref::<f64>().unwrap()),
            DataType::CHAR => Box::new(*(*object).downcast_ref::<char>().unwrap()),
            DataType::BOOL => Box::new(*(*object).downcast_ref::<bool>().unwrap()),
            DataType::STR => Box::new((*object).downcast_ref::<String>().unwrap().clone()),
        };
        return Some(value);
    }

    pub fn any_to_bool(object: &Box<dyn Any>) -> Option<&bool> {
        let data_type = DataType::box_any_to_data_type(object)?;
        let data_type = data_type;
        if data_type != DataType::BOOL {
            return None;
        }
        return Some(object.downcast_ref::<bool>().unwrap());
    }

    pub fn is_are_operands_number(objects: &[&Box<dyn Any>]) -> Result<(), String> {
        for object in objects.iter() {
            match DataType::box_any_to_data_type(*object) {
                Some(data_type) => {
                    if data_type != DataType::INT && data_type != DataType::FLOAT {
                        return Err("Operand must be a number.".to_owned());
                    }
                }
                None => {
                    return Err("Invalid operand data type.".to_owned());
                }
            }
        }
        return Ok(());
    }

    pub fn is_equal(left: &Box<dyn Any>, right: &Box<dyn Any>) -> Result<bool, String> {
        let left_dt = DataType::box_any_to_data_type(left);
        let right_dt = DataType::box_any_to_data_type(right);
        if left_dt.is_none() || right_dt.is_none() {
            return Err("Invalid operand data type.".to_owned());
        }
        let left_dt = left_dt.unwrap();
        let right_dt = right_dt.unwrap();
        // if left_dt != DataType::BOOL && right_dt != DataType::BOOL {
        //     return Err("Operand must be booleans.".to_owned());
        // }
        if left_dt != right_dt {
            return Err(format!(
                "Mismatched types of {:?} and {:?}",
                left_dt, right_dt
            ));
        }
        return match left_dt {
            DataType::INT => {
                let left_value = *left.downcast_ref::<i32>().unwrap();
                let right_value = *right.downcast_ref::<i32>().unwrap();
                Ok(left_value == right_value)
            }
            DataType::FLOAT => {
                let left_value = *left.downcast_ref::<f64>().unwrap();
                let right_value = *right.downcast_ref::<f64>().unwrap();
                Ok(left_value == right_value)
            }
            DataType::CHAR => {
                let left_value = *left.downcast_ref::<char>().unwrap();
                let right_value = *right.downcast_ref::<char>().unwrap();
                Ok(left_value == right_value)
            }
            DataType::BOOL => {
                let left_value = *left.downcast_ref::<bool>().unwrap();
                let right_value = *right.downcast_ref::<bool>().unwrap();
                Ok(left_value == right_value)
            }
            DataType::STR => {
                let left_value = left.downcast_ref::<String>().unwrap();
                let right_value = right.downcast_ref::<String>().unwrap();
                Ok(left_value == right_value)
            }
        };
    }

    pub fn get_default_of_type(token_type: &TokenType) -> Option<Box<dyn Any>> {
        return match token_type {
            TokenType::RkwBool => Some(Box::new(false)),
            TokenType::RkwChar => Some(Box::new('\0')),
            TokenType::RkwFloat => Some(Box::new(0.0)),
            TokenType::RkwInt => Some(Box::new(0)),
            _ => None,
        };
    }

    pub fn get_token_data_type(token: &Token) -> Option<DataType> {
        match token.token_type {
            TokenType::RkwBool => Some(DataType::BOOL),
            TokenType::RkwChar => Some(DataType::CHAR),
            TokenType::RkwFloat => Some(DataType::FLOAT),
            TokenType::RkwInt => Some(DataType::INT),
            _ => None,
        }
    }

    pub fn downcast_box_any<'a, T: 'static>(box_any: &'a Box<dyn Any>) -> Option<&'a T> {
        let mut object = box_any;
        while let Some(value) = (*object).downcast_ref::<Box<dyn Any>>() {
            object = value;
        }

        return (*object).downcast_ref::<T>();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stringify_primitives_i32() {
        assert_eq!(
            Ok(String::from("1")),
            DataType::stringify_primitives(&(Box::new(1) as Box<dyn Any>))
        );
    }

    #[test]
    fn stringify_primitives_f64() {
        assert_eq!(
            Ok(String::from("1.1")),
            DataType::stringify_primitives(&(Box::new(1.1) as Box<dyn Any>))
        );
    }

    #[test]
    fn stringify_primitives_char() {
        assert_eq!(
            Ok(String::from("1")),
            DataType::stringify_primitives(&(Box::new('1') as Box<dyn Any>))
        );
    }

    #[test]
    fn stringify_primitives_bool_true() {
        assert_eq!(
            Ok(String::from("TRUE")),
            DataType::stringify_primitives(&(Box::new(true) as Box<dyn Any>))
        );
    }

    #[test]
    fn stringify_primitives_bool_false() {
        assert_eq!(
            Ok(String::from("FALSE")),
            DataType::stringify_primitives(&(Box::new(false) as Box<dyn Any>))
        );
    }

    #[test]
    fn stringify_primitives_excluded() {
        assert_eq!(
            Err("Invalid data type.".to_owned()),
            DataType::stringify_primitives(&(Box::new("false") as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type.".to_owned()),
            DataType::stringify_primitives(&(Box::new(1 as i8) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type.".to_owned()),
            DataType::stringify_primitives(&(Box::new(1 as i16) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type.".to_owned()),
            DataType::stringify_primitives(&(Box::new(1 as i64) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type.".to_owned()),
            DataType::stringify_primitives(&(Box::new(1 as i128) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type.".to_owned()),
            DataType::stringify_primitives(&(Box::new(1 as f32) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type.".to_owned()),
            DataType::stringify_primitives(&(Box::new(1.1 as f32) as Box<dyn Any>))
        );
    }
}
