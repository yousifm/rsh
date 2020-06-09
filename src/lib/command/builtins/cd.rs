use super::EvalError;
use std::env;

pub fn cd(args: &Vec<String>) -> Result<(), EvalError> {
    if args.len() > 1 {
        Err(EvalError::new("Too many arguments"))
    } else {
        if args.is_empty() {
            env::set_current_dir(env::var("HOME").unwrap()).unwrap();
            return Ok(())
        }

        let home = env::var("HOME").unwrap();
        let dir = &args[0].replace("~", &home);

        match env::set_current_dir(dir) {
            Ok(_) => return Ok(()),
            Err(e) => {
                let code = e.raw_os_error().unwrap(); 
                match code {
                    2 => return Err(EvalError::from_string(format!("Directory \"{}\" does not exist", dir))),
                    20 => return Err(EvalError::from_string(format!("\"{}\" is not a directory", dir))),
                    _ => return Err(EvalError::from_string(e.to_string()))
                }
            }
        }
    }
}