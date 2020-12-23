

use crate::prelude::*;
use crate::lib::*;




pub fn main_execute (_arguments : &[OsString]) -> Outcome<()> {
	
	let mut _options = ExecuteOptions::default ();
	let mut _dump = DumpOptions::default ();
	
	let mut _parser = parser_prepare ();
	_options.parser_prepare (&mut _parser);
	_dump.parser_prepare (&mut _parser);
	parser_execute (&_parser, "execute", _arguments) ?;
	drop (_parser);
	
	let _descriptor = _options.descriptor_build () ?;
	
	if _dump.any () {
		_dump.dump_rust (&_descriptor, None) ?;
		_dump.dump_ron (&_descriptor, None) ?;
		_dump.dump_json (&_descriptor, None) ?;
		return Ok (());
	}
	
	execute (&_descriptor, Some (env::vars_os ())) ?;
	
	fail_assertion! (0x87e0fb65);
}




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
			_ =>
				Err (error (0x8cd8f849, format! ("invalid command `{}`!", _arguments[1].to_string_lossy ()))),
		}
	};
	
	match _outcome {
		Ok (()) =>
			process::exit (0),
		Err (_error) => {
			log_error! (0x5c0e181c, "unexpected error encountered!  aborting!");
			log_error! (0, "{}", _error);
			process::exit (1);
		}
	}
}

