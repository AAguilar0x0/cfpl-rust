use std::any::Any;

#[derive(PartialEq)]
pub enum DataType {
    INT,
    FLOAT,
    CHAR,
    BOOL,
}

impl DataType {
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
}
