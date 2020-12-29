

use crate::prelude::*;
use crate::cmd::*;
use crate::lib::*;




pub fn main_client_1 (_descriptor : ProcessDescriptor, _connect : ServerConnectOptions) -> Outcome<()> {
	
	let _path = if let Some (_path) = _connect.unix_path.as_ref () {
		_path.clone ()
	} else {
		fail! (0xb2fcb5f8, "UNIX domain socket path is required!");
	};
	let _path = &_path;
	
	let mut _socket = rpc_client_connect (_path, None) ?;
	
	return main_client_0 (_descriptor, &mut _socket);
}




pub fn main_client_0 (_descriptor : ProcessDescriptor, _socket : &mut socket2::Socket) -> Outcome<()> {
	
	let _request = RpcExecuteRequest {
			descriptor : _descriptor,
		};
	
	let _response = rpc_client_call (_socket, _request, None) ?;
	
	log_notice! (0x0f4a112f, "spawned process with PID `{}`!", _response.pid);
	
	return Ok (());
}

