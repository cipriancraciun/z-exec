

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
	
	return main_execute_0 (_descriptor, _dump);
}


pub fn main_execute_from (_arguments : &[OsString]) -> Outcome<()> {
	
	let mut _load_path : Option<PathBuf> = None;
	let mut _load_fd : Option<u16> = None;
	let mut _load_format : &str = "json";
	let mut _dump = DumpOptions::default ();
	
	let mut _parser = parser_prepare ();
	_parser.refer (&mut _load_path) .metavar ("<path>")
			.add_option (&["-f", "--from-file"], argparse::StoreOption, "load execution descriptor from given file (else use stdin)");
	_parser.refer (&mut _load_fd) .metavar ("<fd>")
			.add_option (&["--from-fd"], argparse::StoreOption, "load execution descriptor from given file descriptor");
	_parser.refer (&mut _load_format) .metavar ("<format>")
			.add_option (&["--json"], argparse::StoreConst ("json"), "expect JSON serialization")
			.add_option (&["--ron"], argparse::StoreConst ("ron"), "expect RON serialization");
	
	_dump.parser_prepare (&mut _parser);
	parser_execute (&_parser, "execute-from", _arguments) ?;
	drop (_parser);
	
	if _load_path.is_some () && _load_fd.is_some () {
		fail! (0x004aa547, "invalid arguments (both load path and fd given)!");
	}
	
	let mut _stdin = io::stdin ();
	let _load_stream : Box<dyn io::Read> = if let Some (_load_path) = _load_path {
		Box::new (fs::File::open (_load_path) ?)
	} else if let Some (_load_fd) = _load_fd {
		Box::new (unsafe { fs::File::from_raw_fd (_load_fd as io_unix::RawFd) })
	} else {
		Box::new (_stdin.lock ())
	};
	
	let mut _load_stream = io::BufReader::new (_load_stream);
	
	let _descriptor = match _load_format {
		"json" =>
			ProcessDescriptor::json_from_stream (&mut _load_stream) ?,
		"ron" =>
			ProcessDescriptor::ron_from_stream (&mut _load_stream) ?,
		_ =>
			fail_assertion! (0xd49171e2),
	};
	
	drop (_load_stream);
	drop (_stdin);
	
	return main_execute_0 (_descriptor, _dump);
}


pub fn main_execute_0 (_descriptor : ProcessDescriptor, _dump : DumpOptions) -> Outcome<()> {
	
	if _dump.any () {
		_dump.dump_rust (&_descriptor, None) ?;
		_dump.dump_ron (&_descriptor, None) ?;
		_dump.dump_json (&_descriptor, None) ?;
		return Ok (());
	}
	
	execute (&_descriptor, Some (env::vars_os ())) ?;
	
	fail_assertion! (0x117f5a07);
}

