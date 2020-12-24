

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
	let mut _load_stream : Box<dyn io::Read> = if let Some (_load_path) = _load_path {
		Box::new (fs::File::open (_load_path) ?)
	} else if let Some (_load_fd) = _load_fd {
		Box::new (unsafe { fs::File::from_raw_fd (_load_fd as io_unix::RawFd) })
	} else {
		Box::new (_stdin.lock ())
	};
	
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




pub fn main_server (_arguments : &[OsString]) -> Outcome<()> {
	
	let mut _options = ServerOptions::default ();
	
	let mut _parser = parser_prepare ();
	_options.parser_prepare (&mut _parser);
	parser_execute (&_parser, "execute", _arguments) ?;
	drop (_parser);
	
	return main_server_0 (_options);
}


pub fn main_server_0 (_options : ServerOptions) -> Outcome<()> {
	
	let _path = if let Some (_path) = _options.unix_path.as_ref () {
		_path.clone ()
	} else {
		fail! (0x4d548b9c, "UNIX domain socket path is required!");
	};
	let _path = &_path;
	
	let _path_remove = _options.unix_path_remove.unwrap_or (false);
	
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
	
	let _should_stop = {
		let _signals = &[
				signal_sig::SIGINT,
				signal_sig::SIGTERM,
				signal_sig::SIGQUIT,
				signal_sig::SIGHUP,
				signal_sig::SIGPIPE,
				signal_sig::SIGABRT,
			];
		let _flag = sync::Arc::new (atomic::AtomicBool::new (false));
		for &_signal in _signals {
			signal_flag::register (_signal, _flag.clone ()) ?;
		}
		_flag
	};
	
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
			thread::spawn (|| {
					if let Err (_error) = main_server_handle (_socket) {
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




pub fn main_server_handle (_socket : socket2::Socket) -> Outcome<()> {
	fail_unimplemented! (0x85d78136);
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
			b"execute-from" =>
				main_execute_from (&_arguments[2..]),
			b"server" =>
				main_server (&_arguments[2..]),
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

