

use crate::prelude::*;
use crate::cmd::*;
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
	
	let (mut _socket, _socket_metadata) = rpc_server_listen (_path, _path_remove) ?;
	
	let _server = Server::start (&mut _socket, _should_stop) ?;
	_server.wait () ?;
	
	rpc_server_cleanup (_path, _socket, _socket_metadata) ?;
	
	return Ok (());
}

