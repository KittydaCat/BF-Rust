pub mod BF {

    #[derive(Debug)]
    pub enum BFError {
        UnbalancedBrackets,
        NegativeArrayPointer,
        LineReadFailed,
        NonASCIIChar,
        InputFailed,
        OutputFailed
    }

    pub fn run_bf(array: &mut Vec<i32>, mut array_index: usize, instructions: &str,
                  input: &fn() -> Result<String, BFError>,
                  output: &fn(String) -> Result<(), BFError>)
                  -> Result<(), BFError> {
        let mut instruct_index: usize = 0;

        while array_index >= array.len() { array.push(0); } // make sure the index is valid

        'parse_instruct: loop {
            match instructions.chars().nth(instruct_index) {

                Some(instruction) => {(array_index, instruct_index) = exec_instruction(
                    array, array_index, instructions, input, output, instruct_index, instruction)?}
                None => {break 'parse_instruct}
            };
            instruct_index += 1;
        };
        Ok(())
    }

    pub fn exec_instruction(array: &mut Vec<i32>, mut array_index: usize, instructions: &str,
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
                let char = input()?.chars().next().ok_or(BFError::LineReadFailed)?;
                if char.is_ascii() { array[array_index] = char as u8 as i32 }
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

    fn input() -> Result<String, BF::BFError>{

        let mut line = String::new();

        let _ = std::io::stdout().flush();

        match std::io::stdin().read_line(&mut line){

            Ok(_) => {Ok(line)}

            Err(_) => {return Err(BF::BFError::LineReadFailed);}
        }

    }

    fn print(str: String) -> Result<(), BF::BFError>{
        println!("{}",str);
        Ok(())
    }

    let mut file = File::open("foo.txt").expect("File open failed :(");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File read failed :(");

    let mut array = vec![];

    match BF::run_bf(&mut array, 0, &contents,
                     &(input as fn() -> Result<String, BF::BFError>),
                     &(print as fn(String) -> Result<(), BF::BFError>)) {
        Ok(_) => {}
        Err(x) => {println!("Error: {:?}", x);}
    };
    println!("\n{:?}", &array);
}