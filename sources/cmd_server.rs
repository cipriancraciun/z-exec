

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
	
	rpc_server_accept_loop (&mut _socket, &_should_stop, server_handle) ?;
	
	rpc_server_cleanup (_path, _socket, _socket_metadata) ?;
	
	return Ok (());
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




fn server_handle (_socket : socket2::Socket) -> Outcome<()> {
	
	let mut _socket = _socket;
	let _socket = &mut _socket;
	
	log_debug! (0x525491b8, "server begin handling client...");
	
	loop {
		
		log_debug! (0x9c455e79, "server reading request...");
		let _request = rpc_read_or_eof::<RpcRequestWrapper> (_socket) ?;
		let _request = if let Some (_request) = _request {
			_request
		} else {
			break;
		};
		
		match _request {
			RpcRequestWrapper::Execute (_request) =>
				server_handle_execute (_socket, _request) ?,
		}
	}
	
	log_debug! (0x2cac1201, "server finished handling client!");
	
	return Ok (());
}




fn server_handle_execute (_socket : &mut socket2::Socket, _request : RpcExecuteRequest) -> Outcome<()> {
	
	log_debug! (0x36901fe1, "server handling execute...");
	
	let _reply = match spawn (&_request.descriptor, Some (env::vars_os ())) {
		Ok (_pid) =>
			RpcOutcome::Ok (RpcExecuteResponse { pid : _pid }),
		Err (_error) =>
			RpcOutcome::Err (_error.to_string ()),
	};
	
	return rpc_write (_socket, &_reply);
}

