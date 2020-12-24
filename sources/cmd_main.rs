

use crate::prelude::*;
use crate::cmd::*;
use crate::lib::*;




pub fn main () -> ! {
	
	let _arguments = env::args_os () .collect::<Vec<_>> ();
	
	if _arguments.len () <= 1 {
		log_error! (0xcddd152d, "expected command!");
		process::exit (1);
	}
	
	let _outcome = if _arguments[1].as_bytes () .starts_with (b"-") {
		main_execute (&_arguments[1..])
	} else {
		match _arguments[1].as_bytes () {
			
			b"execute" =>
				main_execute (&_arguments[2..]),
			b"execute-from" =>
				main_execute_from (&_arguments[2..]),
			
			b"server" |
			b"server-listen" =>
				main_server_listen (&_arguments[2..]),
			b"server-handle" =>
				main_server_handle (&_arguments[2..]),
			
			_ =>
				Err (error (0x8cd8f849, format! ("invalid command `{}`!", _arguments[1].to_string_lossy ()))),
		}
	};
	
	match _outcome {
		Ok (()) =>
			process::exit (0),
		Err (_error) => {
			log_error! (0x5c0e181c, "unexpected error encountered;  aborting!");
			log_error! (0, "{}", _error);
			process::exit (1);
		}
	}
}

