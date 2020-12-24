

use crate::prelude::*;
use crate::lib::*;




pub fn rpc_server (_path : &Path, _path_remove : bool, _should_stop : sync::Arc<atomic::AtomicBool>, _handler : fn (socket2::Socket) -> Outcome<()>) -> Outcome<()>
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

