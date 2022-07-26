use std::any::Any;

#[derive(PartialEq)]
pub enum DataType {
    INT,
    FLOAT,
    CHAR,
    BOOL,
}

impl DataType {
    pub fn stringify_primitives(object: &Box<dyn Any>) -> Result<String, &'static str> {
        let string = if let Some(output) = object.downcast_ref::<i32>() {
            output.to_string()
        } else if let Some(output) = object.downcast_ref::<f64>() {
            output.to_string()
        } else if let Some(output) = object.downcast_ref::<char>() {
            output.to_string()
        } else if let Some(output) = object.downcast_ref::<bool>() {
            let output_str = output.to_string();
            output_str[0..1].to_uppercase() + &output_str[1..]
        } else {
            return Err("Invalid data type");
        };
        return Ok(string);
    }
    pub fn any_to_data_type(object: &Box<dyn Any>) -> Option<DataType> {
        return if object.is::<i32>() {
            Some(DataType::INT)
        } else if object.is::<f64>() {
            Some(DataType::FLOAT)
        } else if object.is::<char>() {
            Some(DataType::CHAR)
        } else if object.is::<bool>() {
            Some(DataType::BOOL)
        } else {
            None
        };
    }

    pub fn clone_ref_any(object: &Box<dyn Any>) -> Option<Box<dyn Any>> {
        let data_type = DataType::any_to_data_type(object);
        if data_type.is_none() {
            return None;
        }
        let value: Box<dyn Any> = match data_type.unwrap() {
            DataType::INT => Box::new(*object.downcast_ref::<i32>().unwrap()),
            DataType::FLOAT => Box::new(*object.downcast_ref::<f64>().unwrap()),
            DataType::CHAR => Box::new(*object.downcast_ref::<char>().unwrap()),
            DataType::BOOL => Box::new(*object.downcast_ref::<bool>().unwrap()),
        };
        return Some(value);
    }

    pub fn any_to_bool(object: &Box<dyn Any>) -> Option<&bool> {
        let data_type = DataType::any_to_data_type(object);
        if data_type.is_none() {
            return None;
        }
        let data_type = data_type.unwrap();
        if data_type != DataType::BOOL {
            return None;
        }
        return Some(object.downcast_ref::<bool>().unwrap());
    }

    pub fn is_are_operands_number<'a>(objects: &[&Box<dyn Any>]) -> Result<(), &'a str> {
        for object in objects.iter() {
            match DataType::any_to_data_type(*object) {
                Some(data_type) => {
                    if data_type != DataType::INT && data_type != DataType::FLOAT {
                        return Err("Operand must be a number.");
                    }
                }
                None => {
                    return Err("Invalid operand data type.");
                }
            }
        }
        return Ok(());
    }

    pub fn is_equal<'a>(left: &Box<dyn Any>, right: &Box<dyn Any>) -> Result<bool, &'a str> {
        let left_dt = DataType::any_to_data_type(left);
        let right_dt = DataType::any_to_data_type(right);
        if left_dt.is_none() || right_dt.is_none() {
            return Err("Invalid operand data type.");
        }
        let left_dt = left_dt.unwrap();
        let right_dt = right_dt.unwrap();
        if left_dt != DataType::BOOL && right_dt != DataType::BOOL {
            return Err("Operand must be booleans.");
        }
        let left_value = left.downcast_ref::<bool>().unwrap();
        let right_value = right.downcast_ref::<bool>().unwrap();
        return Ok(left_value == right_value);
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
            Ok(String::from("True")),
            DataType::stringify_primitives(&(Box::new(true) as Box<dyn Any>))
        );
    }

    #[test]
    fn stringify_primitives_bool_false() {
        assert_eq!(
            Ok(String::from("False")),
            DataType::stringify_primitives(&(Box::new(false) as Box<dyn Any>))
        );
    }

    #[test]
    fn stringify_primitives_excluded() {
        assert_eq!(
            Err("Invalid data type"),
            DataType::stringify_primitives(&(Box::new("false") as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type"),
            DataType::stringify_primitives(&(Box::new(1 as i8) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type"),
            DataType::stringify_primitives(&(Box::new(1 as i16) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type"),
            DataType::stringify_primitives(&(Box::new(1 as i64) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type"),
            DataType::stringify_primitives(&(Box::new(1 as i128) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type"),
            DataType::stringify_primitives(&(Box::new(1 as f32) as Box<dyn Any>))
        );
        assert_eq!(
            Err("Invalid data type"),
            DataType::stringify_primitives(&(Box::new(1.1 as f32) as Box<dyn Any>))
        );
    }
}
