use std::any::{Any, TypeId};

fn main() {
    cfpl::file("./test_source_codes/0.cfpl");

    // let x0: Box<dyn Any> = Box::new(false);
    // let x1: Box<dyn Any> = Box::new('\0');
    // let x2: Box<dyn Any> = Box::new(0.0);
    // let x3: Box<dyn Any> = Box::new(0);
    // println!("Option: {:?}", Some(0).type_id());
    // println!("Box::new(false): {:?}", (*x0).type_id());
    // println!("Box::new('\0'): {:?}", (*x1).type_id());
    // println!("Box::new(0.0): {:?}", (*x2).type_id());
    // println!("Box::new(0): {:?}", (*x3).type_id());
    // println!("dyn Any: {:?}", TypeId::of::<dyn Any>());
    // println!("Box<dyn Any>: {:?}", TypeId::of::<Box<dyn Any>>());
    // println!("i32: {:?}", TypeId::of::<i32>());
    // println!("f64: {:?}", TypeId::of::<f64>());
    // println!("char: {:?}", TypeId::of::<char>());
    // println!("bool: {:?}", TypeId::of::<bool>());
}
