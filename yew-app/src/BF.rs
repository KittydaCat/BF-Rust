use std::error::Error;
use crate::BF::BF::BFError;

pub mod BF {
    use std::fmt;
    use std::fmt::Formatter;

    #[derive(Debug, Clone)]
    pub enum BFError {
        UnbalancedBrackets,
        NegativeArrayPointer,
        NonASCIIChar,
        InvalidInstructionIndex,
        InputFailed,
        OutputFailed
    }

    impl fmt::Display for BFError{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

            let error_string = match self{
                BFError::UnbalancedBrackets => {"UnbalancedBrackets"}
                BFError::NegativeArrayPointer => {"NegativeArrayPointer"}
                BFError::NonASCIIChar => {"NonASCIIChar"}
                BFError::InvalidInstructionIndex => {"InvalidInstructionIndex"}
                BFError::InputFailed => {"InputFailed"}
                BFError::OutputFailed => {"OutputFailed"}
            };
            write!(f, "{}", error_string)
        }
    }


    pub struct BFInterpreter{
        pub array: Vec<u32>,
        pub array_pointer: usize,

        pub program: String,
        pub program_index: usize,

        pub input: fn() -> Result<String, BFError>,
        pub output: fn(String) -> Result<(), BFError>
    }

    impl BFInterpreter{

        pub fn run(&mut self) -> Result<(), BFError> {

            run_bf(&mut self.array, self.array_pointer, &self.program,
                   &self.input, &self.output, self.program_index)
        }

        pub fn exec_one(&mut self) -> Result<(), BFError> {


            if let Some(instruction) = self.program.chars().nth(self.program_index){

                (self.array_pointer, self.program_index) = exec_BF_instruction(&mut self.array, self.array_pointer, &self.program,
                                    &self.input, &self.output, self.program_index, instruction)?;

                Ok(())
            } else {
                Err(BFError::InvalidInstructionIndex)
            }
        }
    }

    pub fn run_bf(array: &mut Vec<u32>, mut array_index: usize, instructions: &str,
                  input: &fn() -> Result<String, BFError>,
                  output: &fn(String) -> Result<(), BFError>, mut instruct_index: usize)
                  -> Result<(), BFError> {

        while array_index >= array.len() { array.push(0); } // make sure the index is valid

        while let Some(instruction) = instructions.chars().nth(instruct_index) {

            (array_index, instruct_index) = exec_BF_instruction(array, array_index, instructions,
                                                             input, output, instruct_index,
                                                             instruction)?;

            instruct_index += 1;
        }
        Ok(())
    }

    pub fn exec_BF_instruction(array: &mut Vec<u32>, mut array_index: usize, instructions: &str,
                            input: &fn() -> Result<String, BFError>,
                            output: &fn(String) -> Result<(), BFError>,
                            mut instruct_index: usize, instruction: char)
                            -> Result<(usize, usize), BFError> {

        match instruction {

            // increment (>) and decrement (>)
            '+' => { array[array_index] += 1; }
            '-' => { array[array_index] -= 1; }

            // pointer left and right
            '>' => {
                array_index = array_index + 1;

                if array_index == array.len() { array.push(0); } // make sure the index is valid
            }
            '<' => {
                if array_index == 0 { return Err(BFError::NegativeArrayPointer) };
                array_index = array_index - 1;
            }

            // loop stuff
            '[' => {
                if array[array_index] == 0 {
                    instruct_index = equalize_brackets(&instructions, instruct_index, 1)?
                };
            }
            ']' => {
                instruct_index = equalize_brackets(&instructions, instruct_index, -1)? - 1;
            }

            // input (,) and output (.)
            ',' => {
                let char = input()?.chars().next().ok_or(BFError::InputFailed)?;
                if char.is_ascii() { array[array_index] = char as u32 }
                else { return Err(BFError::NonASCIIChar) }
            }
            '.' => {
                if (array[array_index] as u8).is_ascii() {
                    output((array[array_index] as u8 as char).to_string())?
                } else { return Err(BFError::NonASCIIChar) }
            }
            _ => {}
        }

        Ok((array_index, instruct_index))
    }

    fn equalize_brackets(string: &str, mut index: usize, direction: isize) -> Result<usize, BFError> {

        let mut depth = 0;

        'find_next_bracket: loop {

            match string.chars().nth(index) {
                Some('[') => { depth += 1; },
                Some(']') => { depth -= 1; },

                Some(_) => {},

                None => { return Err(BFError::UnbalancedBrackets); }
            };
            if depth == 0 { break 'find_next_bracket };

            index = match index.checked_add_signed(direction) {
                Some(x) => { x },
                None => { return Err(BFError::UnbalancedBrackets); }
            };
        }
        Ok(index)
    }
}

fn main(){

    use std::fs::File;
    use std::io::prelude::*;

    fn input() -> Result<String, BFError>{

        let mut line = String::new();

        let _ = std::io::stdout().flush();

        match std::io::stdin().read_line(&mut line){

            Ok(_) => {Ok(line)}

            Err(_) => {return Err(BFError::InputFailed);}
        }

    }

    fn print(str: String) -> Result<(), BFError>{
        println!("{}",str);
        Ok(())
    }

    let mut file = File::open("foo.txt").expect("File open failed :(");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File read failed :(");

    let mut array = vec![];

    match BF::run_bf(&mut array, 0, &contents,
                     &(input as fn() -> Result<String, BFError>),
                     &(print as fn(String) -> Result<(), BFError>), 0) {
        Ok(_) => {}
        Err(x) => {println!("Error: {:?}", x);}
    };
    println!("\n{:?}", &array);
}