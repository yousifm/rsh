use super::EvalError;
use std::process;

pub fn exit(args: &Vec<String>) -> Result<(), EvalError> {
    if args.len() > 1 {
        Err(EvalError::new("Too many arguments"))
    } else {
        let exit_code: i32;

        if args.len() == 1 {
            let arg_val = &args[0];

            exit_code = match arg_val.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(EvalError::from_string(format!(
                        "Argument '{}' is not a valid integer",
                        arg_val
                    )))
                }
            };
        } else {
            exit_code = 0;
        }
        process::exit(exit_code);
    }
}