use crate::b_unf::BunFError::*;

#[derive(Debug, Clone)]

pub enum Type{
    U32(u32),
    I32(i32),
    Bool(bool),
    Char(u8),
    String(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum EmptyType{
    U32,
    I32,
    Bool,
    Char,
    String,
}

impl From<Type> for EmptyType{
    fn from(value: Type) -> Self {
        match value{
            Type::U32(_) => {EmptyType::U32 }
            Type::I32(_) => {EmptyType::I32 }
            Type::Bool(_) => {EmptyType::Bool }
            Type::Char(_) => {EmptyType::Char }
            Type::String(_) => {EmptyType::String }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BunFError{
    TypeMismatch(Vec<Option<EmptyType>>, Vec<Option<EmptyType>>), // expected ... found ...
}

pub struct BunF{
    pub array: Vec<Type>,
    pub output: String,
}

impl BunF{

    pub fn new() -> Self{
        Self{array: vec![], output: String::new()}
    }

    pub fn push(&mut self, item: Type){

        self.output.push_str(&*match &item { // TODO: Add BF code labeling?

            Type::U32(n) => {format!(">{}", "+".repeat(*n as usize))}

            Type::I32(x) => {
                let mut output = String::from(">");

                if x.is_negative(){output.push_str("+")} // TODO: Think if good decision
                else{output.push_str("")};

                output.push_str(&*format!(">{}", "+".repeat(x.abs() as usize)));

                output

            }
            Type::Bool(x) => {format!(">{}", {if *x{"+"} else{""}})}
            Type::Char(char) => {format!(">{}", "+".repeat(*char as usize))}
            Type::String(str) => {str.iter().map(|char|format!(">{}", "+".repeat(*char as usize))).collect::<String>()}
        });

        self.array.push(item);
    }

    pub fn pop(&mut self) -> Result<Type, BunFError>{

        self.output.push_str(&match self.array.last().ok_or(TypeMismatch(vec!(None),vec!(None)))?{

            Type::U32(_) | Type::Bool(_) | Type::Char(_) => {String::from("[-]<")}
            Type::I32(_) => {String::from("[-]<[-]<")}
            Type::String(x) => {"[-]<".repeat(x.len())}
        });

        self.array.pop().ok_or(TypeMismatch(vec!(None),vec!(None)))
    }

    pub fn add_u32(&mut self) -> Result<(),BunFError>{

        match &self.array[..] {
            [.., Type::U32(x), Type::U32(y)] => {

                let (x, y) = (x.clone(), y.clone());

                self.output.push_str("[-<+>]<");

                self.array.pop();
                *self.array.last_mut().expect("Array should have at least two items") = Type::U32(x+y);
                Ok(())
            },
            [.., x, y] => {
                Err(TypeMismatch(vec!(Some(EmptyType::U32), Some(EmptyType::U32)),
                                 vec!(Some(EmptyType::from(x.clone())), Some(EmptyType::from(y.clone())))))
            }
            [x] => {
                Err(TypeMismatch(vec!(Some(EmptyType::U32), Some(EmptyType::U32)),
                                 vec!(Some(EmptyType::from(x.clone())), None)))
            }
            [] => {
                Err(TypeMismatch(vec!(Some(EmptyType::U32), Some(EmptyType::U32)),
                                 vec!(None, None)))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{BunF,Type};
    #[test]
    fn test_string(){
        let mut x = BunF::new();

        x.push(Type::String(vec!['a', 'b', 'c'].iter().map(|&char| char as u8).collect()));

        println!("{}", x.output)
    }
}