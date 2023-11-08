use crate::b_unf::BunFError::*;

#[derive(Debug, Clone)]

pub enum Type{
    u32(u32),
    i32(i32),
    bool(bool),
    char(u8),
    string(Vec<u8>),
}

#[derive(Debug, Clone, Copy)]
enum EmptyType{
    u32,
    i32,
    bool,
    char,
    string,
}

impl From<Type> for EmptyType{
    fn from(value: Type) -> Self {
        match value{
            Type::u32(_) => {EmptyType::u32}
            Type::i32(_) => {EmptyType::i32}
            Type::bool(_) => {EmptyType::bool}
            Type::char(_) => {EmptyType::char}
            Type::string(_) => {EmptyType::string}
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

            Type::u32(n) => {format!(">{}", "+".repeat(*n as usize))}

            Type::i32(x) => {
                let mut output = String::from(">");

                if x.is_negative(){output.push_str("+")} // TODO: Think if good decision
                else{output.push_str("")};

                output.push_str(&*format!(">{}", "+".repeat(x.abs() as usize)));

                output

            }
            Type::bool(x) => {format!(">{}", {if *x{"+"} else{""}})}
            Type::char(char) => {format!(">{}","+".repeat(*char as usize))}
            Type::string(str) => {str.iter().map(|char|format!(">{}","+".repeat(*char as usize))).collect::<String>()}
        });

        self.array.push(item);
    }

    pub fn pop(&mut self) -> Result<Type, BunFError>{

        self.output.push_str(&match self.array.last().ok_or(TypeMismatch(vec!(None),vec!(None)))?{

            Type::u32(_) | Type::bool(_) | Type::char(_) => {String::from("[-]<")}
            Type::i32(_) => {String::from("[-]<[-]<")}
            Type::string(x) => {"[-]<".repeat(x.len())}
        });

        self.array.pop().ok_or(TypeMismatch(vec!(None),vec!(None)))
    }

    pub fn add_u32(&mut self) -> Result<(),BunFError>{

        match &self.array[..] {
            [.., Type::u32(x), Type::u32(y)] => {

                self.output.push_str("[-<+>]<");
                let (x, y) = (*x, *y);

                self.array.pop();
                *self.array.last_mut().expect("Array should have at least two items") = Type::u32(x+y);
                Ok(())
            },
            [.., x, y] => {
                Err(TypeMismatch(vec!(Some(EmptyType::u32),Some(EmptyType::u32)),
                                 vec!(Some(EmptyType::from(x.clone())), Some(EmptyType::from(y.clone())))))
            }
            [x] => {
                Err(TypeMismatch(vec!(Some(EmptyType::u32),Some(EmptyType::u32)),
                                 vec!(Some(EmptyType::from(x.clone())), None)))
            }
            [] => {
                Err(TypeMismatch(vec!(Some(EmptyType::u32),Some(EmptyType::u32)),
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

        x.push(Type::string(vec!['a','b','c'].iter().map(|&char| char as u8).collect()));

        println!("{}", x.output)
    }
}