#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Integer(i64),
    Array(Box<Array>),
    Tuple(Box<TwoTuple>),
}

#[derive(Debug)]
pub struct TwoTuple {
    values: Vec<Value>
}

#[derive(Debug)]
pub struct Array {
    values: Vec<Value>
}

#[cfg(test)]
mod tests {
    fn it_words() {

    }
}
