use std::{any::Any, fmt::Display};

use crate::{data_type::DataType, environment::Environment, token::Token, token_type::TokenType};

use super::Expression;

pub struct Binary {
    pub operator: Token,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Binary {
    fn visit(&self, environment: &mut Environment) -> Result<Box<dyn std::any::Any>, String> {
        type TupleOkResult = (Box<dyn Any>, DataType, Box<dyn Any>, DataType);
        let get_values_data_type =
            |environment: &mut Environment| -> Result<TupleOkResult, String> {
                let left_value = self.left.visit(environment)?;
                let right_value = self.right.visit(environment)?;
                DataType::is_are_operands_number(&[&left_value, &right_value])?;
                let left_dt = DataType::box_any_to_data_type(&left_value);
                let right_dt = DataType::box_any_to_data_type(&right_value);
                if left_dt.is_none() || right_dt.is_none() {
                    return Err("Invalid operand data type.".to_owned());
                }
                Ok((left_value, left_dt.unwrap(), right_value, right_dt.unwrap()))
            };
        match self.operator.token_type {
            TokenType::SymGreater => {
                let (left_value, left_dt, right_value, right_dt) =
                    get_values_data_type(environment)?;
                if left_dt == DataType::FLOAT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(left_value > right_value));
                } else if left_dt == DataType::INT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(left_value > right_value));
                } else if left_dt == DataType::FLOAT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(*left_value > f64::from(*right_value)));
                } else if left_dt == DataType::INT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(f64::from(*left_value) > *right_value));
                } else {
                    return Err("Operand must be a number.".to_owned());
                }
            }
            TokenType::SymGreaterEqual => {
                let (left_value, left_dt, right_value, right_dt) =
                    get_values_data_type(environment)?;
                if left_dt == DataType::FLOAT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(left_value >= right_value));
                } else if left_dt == DataType::INT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(left_value >= right_value));
                } else if left_dt == DataType::FLOAT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(*left_value >= f64::from(*right_value)));
                } else if left_dt == DataType::INT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(f64::from(*left_value) >= *right_value));
                } else {
                    return Err("Operand must be a number.".to_owned());
                }
            }
            TokenType::SymLesser => {
                let (left_value, left_dt, right_value, right_dt) =
                    get_values_data_type(environment)?;
                if left_dt == DataType::FLOAT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(left_value < right_value));
                } else if left_dt == DataType::INT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(left_value < right_value));
                } else if left_dt == DataType::FLOAT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(*left_value < f64::from(*right_value)));
                } else if left_dt == DataType::INT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(f64::from(*left_value) < *right_value));
                } else {
                    return Err("Operand must be a number.".to_owned());
                }
            }
            TokenType::SymLesserEqual => {
                let (left_value, left_dt, right_value, right_dt) =
                    get_values_data_type(environment)?;
                if left_dt == DataType::FLOAT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(left_value <= right_value));
                } else if left_dt == DataType::INT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(left_value <= right_value));
                } else if left_dt == DataType::FLOAT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(*left_value <= f64::from(*right_value)));
                } else if left_dt == DataType::INT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(f64::from(*left_value) <= *right_value));
                } else {
                    return Err("Operand must be a number.".to_owned());
                }
            }
            TokenType::SymNotEqual => {
                return Ok(Box::new(!DataType::is_equal(
                    &self.left.visit(environment)?,
                    &self.right.visit(environment)?,
                )?));
            }
            TokenType::SymEqual => {
                return Ok(Box::new(DataType::is_equal(
                    &self.left.visit(environment)?,
                    &self.right.visit(environment)?,
                )?));
            }
            TokenType::SymMinus => {
                let (left_value, left_dt, right_value, right_dt) =
                    get_values_data_type(environment)?;
                if left_dt == DataType::FLOAT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(left_value - right_value));
                } else if left_dt == DataType::INT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(left_value - right_value));
                } else if left_dt == DataType::FLOAT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(*left_value - f64::from(*right_value)));
                } else if left_dt == DataType::INT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(f64::from(*left_value) - *right_value));
                } else {
                    return Err("Operand must be a number.".to_owned());
                }
            }
            TokenType::SymPlus => {
                let (left_value, left_dt, right_value, right_dt) =
                    get_values_data_type(environment)?;
                if left_dt == DataType::FLOAT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(left_value + right_value));
                } else if left_dt == DataType::INT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(left_value + right_value));
                } else if left_dt == DataType::FLOAT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(*left_value + f64::from(*right_value)));
                } else if left_dt == DataType::INT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(f64::from(*left_value) + *right_value));
                } else {
                    return Err("Operand must be a number.".to_owned());
                }
            }
            TokenType::SymForwardSlash => {
                let (left_value, left_dt, right_value, right_dt) =
                    get_values_data_type(environment)?;
                if left_dt == DataType::FLOAT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(left_value / right_value));
                } else if left_dt == DataType::INT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(left_value / right_value));
                } else if left_dt == DataType::FLOAT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(*left_value / f64::from(*right_value)));
                } else if left_dt == DataType::INT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(f64::from(*left_value) / *right_value));
                } else {
                    return Err("Operand must be a number.".to_owned());
                }
            }
            TokenType::SymStar => {
                let (left_value, left_dt, right_value, right_dt) =
                    get_values_data_type(environment)?;
                if left_dt == DataType::FLOAT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(left_value * right_value));
                } else if left_dt == DataType::INT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(left_value * right_value));
                } else if left_dt == DataType::FLOAT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<f64>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(*left_value * f64::from(*right_value)));
                } else if left_dt == DataType::INT && right_dt == DataType::FLOAT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<f64>().unwrap();
                    return Ok(Box::new(f64::from(*left_value) * *right_value));
                } else {
                    return Err("Operand must be a number.".to_owned());
                }
            }
            TokenType::SymPercent => {
                let (left_value, left_dt, right_value, right_dt) =
                    get_values_data_type(environment)?;
                if left_dt == DataType::INT && right_dt == DataType::INT {
                    let left_value = left_value.downcast_ref::<i32>().unwrap();
                    let right_value = right_value.downcast_ref::<i32>().unwrap();
                    return Ok(Box::new(left_value % right_value));
                } else {
                    return Err("Operand must be a number.".to_owned());
                }
            }
            TokenType::SymAmpersand => {
                return Ok(Box::new(
                    DataType::stringify_primitives(&self.left.visit(environment)?)?
                        + &DataType::stringify_primitives(&self.right.visit(environment)?)?,
                ));
            }
            _ => return Err("Invalid unary operator.".to_owned()),
        };
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binary(Expression, {:?}, Expression)", self.operator)
    }
}
