

use crate::prelude::*;
use crate::lib::*;




pub fn rpc_server_loop (_path : &Path, _path_remove : bool, _should_stop : sync::Arc<atomic::AtomicBool>, _handler : fn (socket2::Socket) -> Outcome<()>) -> Outcome<()>
{
	if _path.exists () {
		if _path_remove {
			log_warning! (0x256766df, "server socket path already exists: `{}`;  removing!", _path.display ());
			if let Err (_error) = fs::remove_file (_path) {
				log_error! (0x90ad8b77, "unexpected error encountered;  ignoring!  //  {}", _error);
			}
		} else {
			fail! (0x9b835bb0, "server socket path already exists: `{}`;  aborting!", _path.display ());
		}
	}
	if let Some (_parent) = _path.parent () {
		if ! _parent.exists () {
			fail! (0x07e3e056, "server socket path parent does not exist:  `{}`;  aborting!", _parent.display ());
		}
	} else {
		fail_assertion! (0x28eb84fc);
	}
	
	let _address = socket2::SockAddr::unix (_path) ?;
	
	let _should_wait = crossbeam_sync::WaitGroup::new ();
	
	let mut _socket = socket2::Socket::new (
			socket2::Domain::unix (),
			socket2::Type::seqpacket () .cloexec (),
			None
		) ?;
	
	_socket.set_read_timeout (Some (time::Duration::from_millis (250))) ?;
	
	log_debug! (0x24298d86, "server socket binding on `{}`...", _path.display ());
	_socket.bind (&_address) ?;
	
	log_debug! (0xf2d63f9b, "server socket listening...");
	_socket.listen (1024) ?;
	
	scopeguard::defer! {
		if let Err (_error) = fs::remove_file (_path) {
			log_error! (0x52d37764, "unexpected error encountered;  ignoring!  //  {}", _error);
		}
	}
	
	log_debug! (0xbf2564c9, "server socket accepting...");
	
	loop {
		
		if _should_stop.load (atomic::Ordering::Relaxed) {
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
						fail_wrap! (0x39fa3406, "failed accepting!", _error),
				}
		};
		
		{
			let _should_wait = _should_wait.clone ();
			thread::spawn (move || {
					if let Err (_error) = _handler (_socket) {
						log_error! (0x4f8ecc5c, "unexpected error encountered;  ignoring!  //  {}", _error);
					}
					drop (_should_wait);
				});
		}
		
		log_debug! (0xbf2564c9, "server socket accepting...");
	}
	
	log_debug! (0x11cbd18f, "server threads joining...");
	_should_wait.wait ();
	
	log_debug! (0x38ace2b8, "server exiting!");
	
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
	//		fail_wrap! (0x5aa2eca3, "failed decoding RPC message!", _error),
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
	//		fail_wrap! (0x4c224ae4, "failed encoding RPC message!", _error),
	//}
	
	let _buffer = _buffer.into_inner ();
	
	log_debug! (0xafd88ce8, "sending RPC message of {} bytes...", _buffer.len ());
	
	let _sent = _socket.send (_buffer.deref ()) ?;
	if _sent != _buffer.len () {
		fail! (0x39a8d8cf, "failed sending RPC message (buffer truncated)!");
	}
	
	return Ok (());
}

