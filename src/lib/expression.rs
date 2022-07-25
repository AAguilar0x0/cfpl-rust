use std::any::Any;

pub mod variable;

pub trait Expression {
    fn visit(&self) -> Box<dyn Any>;
}

pub fn stringify_primitives<'a>(object: Box<dyn Any>) -> Result<String, &'a str> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stringify_primitives_i32() {
        assert_eq!(Ok(String::from("1")), stringify_primitives(Box::new(1)));
    }

    #[test]
    fn stringify_primitives_f64() {
        assert_eq!(Ok(String::from("1.1")), stringify_primitives(Box::new(1.1)));
    }

    #[test]
    fn stringify_primitives_char() {
        assert_eq!(Ok(String::from("1")), stringify_primitives(Box::new('1')));
    }

    #[test]
    fn stringify_primitives_bool_true() {
        assert_eq!(
            Ok(String::from("True")),
            stringify_primitives(Box::new(true))
        );
    }

    #[test]
    fn stringify_primitives_bool_false() {
        assert_eq!(
            Ok(String::from("False")),
            stringify_primitives(Box::new(false))
        );
    }

    #[test]
    fn stringify_primitives_excluded() {
        assert_eq!(
            Err("Invalid data type"),
            stringify_primitives(Box::new("false"))
        );
        assert_eq!(
            Err("Invalid data type"),
            stringify_primitives(Box::new(1 as i8))
        );
        assert_eq!(
            Err("Invalid data type"),
            stringify_primitives(Box::new(1 as i16))
        );
        assert_eq!(
            Err("Invalid data type"),
            stringify_primitives(Box::new(1 as i64))
        );
        assert_eq!(
            Err("Invalid data type"),
            stringify_primitives(Box::new(1 as i128))
        );
        assert_eq!(
            Err("Invalid data type"),
            stringify_primitives(Box::new(1 as f32))
        );
        assert_eq!(
            Err("Invalid data type"),
            stringify_primitives(Box::new(1.1 as f32))
        );
    }
}
