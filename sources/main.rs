

use crate::prelude::*;
use crate::lib::*;




pub fn main_0 () -> Result<(), io::Error> {
	
	fail_unimplemented! (0x49a548b5);
}




pub fn main () -> ! {
	
	let _arguments = env::args_os () .collect::<Vec<_>> ();
	
	if _arguments.len () != 1 {
		log_error! (0x1ee2a8c2, "invalid arguments count!");
		process::exit (1);
	}
	
	match main_0 () {
		Ok (()) =>
			process::exit (0),
		Err (_error) => {
			log_error! (0x5c0e181c, "unexpected error encountered!  aborting!");
			log_error! (0, "{}", _error);
			process::exit (1);
		}
	}
}

