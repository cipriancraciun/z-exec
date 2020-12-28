

use crate::prelude::*;
use crate::lib::*;




pub fn rpc_server_listen (_path : &Path, _path_remove : bool) -> Outcome<(socket2::Socket, fs::Metadata)> {
	
	log_debug! (0x788abc1f, "server socket listening...");
	
	if let Some (_parent) = _path.parent () {
		if ! _parent.exists () {
			fail! (0x07e3e056, "server socket path parent does not exist: `{}`;  aborting!", _parent.display ());
		}
	} else {
		fail_assertion! (0x28eb84fc);
	}
	
	rpc_server_socket_remove (_path, Some (_path_remove), None) ?;
	
	let _address = socket2::SockAddr::unix (_path) ?;
	
	log_debug! (0xa0daab13, "server socket creating...");
	let mut _socket = socket2::Socket::new (
			socket2::Domain::unix (),
			socket2::Type::seqpacket () .cloexec (),
			None
		) ?;
	
	log_information! (0x24298d86, "server socket listening on: `{}`...", _path.display ());
	
	_socket.bind (&_address) ?;
	_socket.listen (1024) ?;
	
	let _socket_metadata = fs::symlink_metadata (_path) ?;
	if ! _socket_metadata.file_type () .is_socket () {
		fail! (0x8f0c5694, "server socket path exists, but is not a socket: `{}`;  aborting!", _path.display ());
	}
	
	log_debug! (0x41682e69, "server socket listening succeeded;");
	
	return Ok ((_socket, _socket_metadata));
}


pub fn rpc_server_cleanup (_path : &Path, _socket : socket2::Socket, _socket_metadata : fs::Metadata) -> Outcome<()> {
	
	log_debug! (0x54dc5a73, "server socket cleaning...");
	
	log_debug! (0x68e80478, "server socket unlinking from: `{}`...", _path.display ());
	rpc_server_socket_remove (_path, None, Some (_socket_metadata)) .or_log_error (0x2696eb91);
	
	log_debug! (0x61bdb980, "server socket destroying...");
	_socket.shutdown (net::Shutdown::Both) .or_log_error (0x92474573);
	
	drop (_socket);
	
	log_debug! (0xb4413093, "server socket cleaning finished;");
	
	return Ok (());
}




pub fn rpc_server_accept (_socket : &mut socket2::Socket, _should_stop : SyncTrigger, _handler : fn (socket2::Socket) -> Outcome<()>) -> Outcome<()> {
	
	log_debug! (0xbf2564c9, "server socket accepting...");
	
	let _should_wait = crossbeam_sync::WaitGroup::new ();
	
	_socket.set_read_timeout (Some (time::Duration::from_millis (250))) ?;
	
	loop {
		
		if _should_stop.is_triggered () {
			log_debug! (0x5d8283e8, "server socket breaking...");
			break;
		}
		
		let _socket = match _socket.accept () {
			Ok ((_socket, _)) =>
				_socket,
			Err (_error) =>
				match _error.raw_os_error () .map (nix::Errno::from_i32) {
					Some (nix::EAGAIN) =>
						continue,
					_ =>
						fail_wrap! (0x39fa3406, _error),
				}
		};
		
		log_debug! (0x7b8fad6d, "server socket client accepted;");
		
		{
			let _should_wait = _should_wait.clone ();
			thread::spawn (move || {
					log_debug! (0x4e1ff251, "server socket client handling...");
					_handler (_socket) .or_log_error (0x9f1efcd5);
					drop (_should_wait);
					log_debug! (0xcd929c16, "server socket client handled;");
				});
		}
		
		log_debug! (0xbf2564c9, "server socket accepting...");
	}
	
	log_debug! (0x11cbd18f, "server socket joining...");
	_should_wait.wait ();
	
	log_debug! (0x7c53687d, "server socket accepting finished;");
	
	return Ok (());
}




pub fn rpc_client_connect (_path : &Path) -> Outcome<socket2::Socket> {
	
	let _address = socket2::SockAddr::unix (_path) ?;
	
	let mut _socket = socket2::Socket::new (
			socket2::Domain::unix (),
			socket2::Type::seqpacket () .cloexec (),
			None
		) ?;
	
	_socket.set_read_timeout (Some (time::Duration::from_millis (250))) ?;
	
	_socket.connect (&_address) ?;
	
	return Ok (_socket);
}


pub fn rpc_client_call <Request : RpcRequest<Response = Response>, Response : RpcResponse> (_socket : &mut socket2::Socket, _request : Request) -> Outcome<Response> {
	
	let _request = _request.wrap ();
	rpc_write (_socket, &_request) ?;
	
	match rpc_read::<RpcOutcome<Response>> (_socket) ? {
		RpcOutcome::Ok (_response) =>
			return Ok (_response),
		RpcOutcome::Err (_message) =>
			return Err (io::Error::new (io::ErrorKind::Other, _message)),
	}
}




pub fn rpc_read <Payload : Serializable> (_socket : &mut socket2::Socket) -> Outcome<Payload> {
	match rpc_read_or_eof::<Payload> (_socket) ? {
		Some (_payload) =>
			return Ok (_payload),
		None =>
			fail! (0x1c8753b2, "failed receiving RPC message (socket closed)!"),
	}
}


pub fn rpc_read_or_eof <Payload : Serializable> (_socket : &mut socket2::Socket) -> Outcome<Option<Payload>> {
	
	use bytes::Buf;
	
	let mut _buffer = bytes::BytesMut::with_capacity (RPC_BUFFER_SIZE);
	unsafe { _buffer.set_len (RPC_BUFFER_SIZE); }
	
	// NOTE:  We are using UNIX domain sockets of type sequence packets, thus packet boundary is solved by the OS.
	let _received = _socket.recv (_buffer.deref_mut ()) ?;
	if _received == 0 {
		return Ok (None);
	}
	if _received < 1 {
		fail! (0x2f2e7dc8, "failed receiving RPC message (buffer truncated)!");
	}
	_buffer.truncate (_received);
	
	log_debug! (0x9daaaaf4, "received RPC message of {} bytes...", _buffer.len ());
	
	let mut _buffer = _buffer.reader ();
	let _payload = Payload::json_from_stream (&mut _buffer) ?;
	
	// FIXME!
	//let _payload = match serde_bincode::deserialize_from::<_, Payload> (&mut _buffer) {
	//	Ok (_payload) =>
	//		_payload,
	//	Err (_error) =>
	//		fail_wrap! (0x5aa2eca3, _error),
	//};
	
	let _buffer = _buffer.into_inner ();
	
	if ! _buffer.is_empty () {
		fail! (0x5322b1da, "failed decoding RPC message (buffer garbage)!");
	}
	
	return Ok (Some (_payload));
}


pub fn rpc_write <Payload : Serializable> (_socket : &mut socket2::Socket, _payload : &Payload) -> Outcome<()> {
	
	use bytes::BufMut;
	
	let _buffer = bytes::BytesMut::with_capacity (RPC_BUFFER_SIZE);
	
	let mut _buffer = _buffer.writer ();
	_payload.json_into_stream (&mut _buffer, false) ?;
	
	// FIXME!
	//match serde_bincode::serialize_into (&mut _buffer, _payload) {
	//	Ok (()) =>
	//		(),
	//	Err (_error) =>
	//		fail_wrap! (0x4c224ae4, _error),
	//}
	
	let _buffer = _buffer.into_inner ();
	
	log_debug! (0xafd88ce8, "sending RPC message of {} bytes...", _buffer.len ());
	
	let _sent = _socket.send (_buffer.deref ()) ?;
	if _sent != _buffer.len () {
		fail! (0x39a8d8cf, "failed sending RPC message (buffer truncated)!");
	}
	
	return Ok (());
}




fn rpc_server_socket_remove (_path : &Path, _path_remove : Option<bool>, _expected_metadata : Option<fs::Metadata>) -> Outcome<()> {
	
	match fs::symlink_metadata (_path) {
		
		Ok (_path_metadata) => {
			
			if _path_metadata.file_type () .is_socket () {
				
				let _allow_remove = if let Some (_expected_metadata) = _expected_metadata {
					if (_path_metadata.dev () == _expected_metadata.dev ()) && (_path_metadata.ino () == _expected_metadata.ino ()) {
						true
					} else {
						log_warning! (0x7b0c8a32, "server socket path exists, but does not match expected: `{}`!", _path.display ());
						false
					}
				} else {
					true
				};
				
				if _allow_remove && _path_remove.unwrap_or (true) {
					
					if _path_remove.is_some () {
						log_warning! (0x256766df, "server socket path exists: `{}`;  removing!", _path.display ());
					}
					
					fs::remove_file (_path) ?;
					
				} else {
					
					if _path_remove.is_some () {
						fail! (0x9b835bb0, "server socket path exists: `{}`!", _path.display ());
					}
				}
				
			} else {
				fail! (0x9b835bb0, "server socket path exists, but is not a socket: `{}`;  aborting!", _path.display ());
			}
		}
		
		Err (_error) =>
			match _error.kind () {
				io::ErrorKind::NotFound =>
					log_debug! (0x22dc2d03, "server socket path does not exist: `{}`;  ignoring!", _path.display ()),
				_ =>
					fail_wrap! (0xfb94f36f, _error),
			}
	}
	
	return Ok (());
}

