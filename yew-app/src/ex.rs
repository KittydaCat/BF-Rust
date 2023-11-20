use std::cell::RefCell;
use std::rc::Rc;

enum Error{
    InputError,
    //...
}

struct BFInterp {
    input: Box<dyn FnMut() -> Result<char, Error>>,
    // ...
}

struct Website{
    input: Rc<RefCell<Option<char>>>,
    interp: BFInterp,
    // ...
}

impl Website {

    pub fn new() -> Self{

        let input_val: Rc<RefCell<Option<char>>> = Rc::from(RefCell::from(None));

        let input =  move || -> Result<char, Error> {

            if let Some(char) = *input_val.borrow(){
                Ok(char)
            } else {Err(Error::InputError)}
        };

        Self{
            input: input_val,
            interp: BFInterp {
                input: Box::new(input)
            },
        }
    }

}