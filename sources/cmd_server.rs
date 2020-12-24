

use crate::prelude::*;
use crate::lib::*;




pub fn main_server_listen (_arguments : &[OsString]) -> Outcome<()> {
	
	let mut _options = ServerListenOptions::default ();
	
	let mut _parser = parser_prepare ();
	_options.parser_prepare (&mut _parser);
	parser_execute (&_parser, "server-listen", _arguments) ?;
	drop (_parser);
	
	return main_server_listen_0 (_options);
}


pub fn main_server_listen_0 (_options : ServerListenOptions) -> Outcome<()> {
	
	let _path = if let Some (_path) = _options.unix_path.as_ref () {
		_path.clone ()
	} else {
		fail! (0x4d548b9c, "UNIX domain socket path is required!");
	};
	let _path = &_path;
	
	let _path_remove = _options.unix_path_remove.unwrap_or (false);
	
	let _should_stop = should_stop ();
	
	return rpc_server (_path, _path_remove, _should_stop, main_server_listen_handle);
}


fn main_server_listen_handle (_socket : socket2::Socket) -> Outcome<()> {
	fail_unimplemented! (0xf4a6475a);
}




pub fn main_server_handle (_arguments : &[OsString]) -> Outcome<()> {
	
	let mut _options = ServerHandleOptions::default ();
	
	let mut _parser = parser_prepare ();
	_options.parser_prepare (&mut _parser);
	parser_execute (&_parser, "server-handle", _arguments) ?;
	drop (_parser);
	
	return main_server_handle_0 (_options);
}

pub fn main_server_handle_0 (_options : ServerHandleOptions) -> Outcome<()> {
	fail_unimplemented! (0x85d78136);
}

