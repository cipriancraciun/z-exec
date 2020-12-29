

use crate::prelude::*;
use crate::lib::*;




pub struct Server {
	
	context : SyncBox<ServerContext>,
	state : SyncBox<ServerState>,
	threads : ServerThreads,
}


struct ServerState {
	
	processes : HashMap<Identifier, SyncBox<ProcessState>>,
	processes_by_pid : HashMap<ProcessId, Identifier>,
}


struct ServerContext {
	
	handler_sender : SyncCallSender<RpcRequestWrapper, RpcOutcomeBox>,
	spawner_sender : SyncCallSender<ProcessDescriptor, Outcome<SyncBox<ProcessState>>>,
	
	spawn_stdin : Option<fs::File>,
	spawn_stdout : Option<fs::File>,
	spawn_stderr : Option<fs::File>,
}


struct ServerThreads {
	
	accepter_thread : thread::JoinHandle<Outcome>,
	handler_thread : thread::JoinHandle<Outcome>,
	spawner_thread : thread::JoinHandle<Outcome>,
	ripper_thread : thread::JoinHandle<Outcome>,
	
	stopper : SyncTrigger,
	waiter : crossbeam_sync::WaitGroup,
}


struct ProcessState {
	
	identifier : Identifier,
	pid : ProcessId,
	outcome : Option<ProcessOutcome>,
}




struct ThreadState {
	
	context : SyncBox<ServerContext>,
	state : SyncBox<ServerState>,
	
	stopper : SyncTrigger,
	waiter : crossbeam_sync::WaitGroup,
}


impl ThreadState {
	
	fn should_stop (&self) -> bool {
		return self.stopper.is_triggered ();
	}
	
	fn sleep (&self) -> () {
		thread::sleep (time::Duration::from_millis (100));
	}
	
	fn clone (&self) -> Self {
		return ThreadState {
				context : self.context.clone (),
				state : self.state.clone (),
				stopper : self.stopper.clone (),
				waiter : self.waiter.clone (),
			};
	}
}




impl Server {
	
	pub fn start (_socket : &mut socket2::Socket, _stopper : SyncTrigger) -> Outcome<Server> {
		
		let _socket = _socket.try_clone () .or_wrap (0x8c84e649) ?;
		
		let _spawn_stdin = fs::OpenOptions::new () .read (true) .open ("/dev/null") .or_panic (0x59145fc4);
		let _spawn_stdout = fs::OpenOptions::new () .append (true) .open ("/dev/stderr") .or_panic (0x79ff9047);
		let _spawn_stderr = fs::OpenOptions::new () .append (true) .open ("/dev/stderr") .or_panic (0x756fdc00);
		
		let _waiter = crossbeam_sync::WaitGroup::new ();
		
		let (_handler_sender, _handler_receiver) = sync_call_new ();
		let (_spawner_sender, _spawner_receiver) = sync_call_new ();
		
		let _context = ServerContext {
				handler_sender : _handler_sender,
				spawner_sender : _spawner_sender,
				spawn_stdin : Some (_spawn_stdin),
				spawn_stdout : Some (_spawn_stdout),
				spawn_stderr : Some (_spawn_stderr),
			};
		
		let _state = ServerState {
				processes : HashMap::with_capacity (1024),
				processes_by_pid : HashMap::with_capacity (1024),
			};
		
		let _context = SyncBox::new (_context);
		let _state = SyncBox::new (_state);
		
		let _thread_state = ThreadState {
				context : _context.clone (),
				state : _state.clone (),
				stopper : _stopper.clone (),
				waiter : _waiter.clone (),
			};
		
		let _accepter_thread = {
			let _thread_state = _thread_state.clone ();
			thread_spawn ("z-exec.server.accepter", move || server_accepter_loop (_thread_state, _socket)) .or_panic (0x6d515f7e)
		};
		
		let _handler_thread = {
			let _thread_state = _thread_state.clone ();
			thread_spawn ("z-exec.server.handler", move || server_handler_loop (_thread_state, _handler_receiver)) .or_panic (0xea492bcc)
		};
		
		let _spawner_thread = {
			let _thread_state = _thread_state.clone ();
			thread_spawn ("z-exec.server.spawner", move || server_spawner_loop (_thread_state, _spawner_receiver)) .or_panic (0xea492bcc)
		};
		
		let _ripper_thread = {
			let _thread_state = _thread_state.clone ();
			thread_spawn ("z-exec.server.ripper", move || server_ripper_loop (_thread_state)) .or_panic (0x37277e58)
		};
		
		let _threads = ServerThreads {
				accepter_thread : _accepter_thread,
				handler_thread : _handler_thread,
				spawner_thread : _spawner_thread,
				ripper_thread : _ripper_thread,
				stopper : _stopper,
				waiter : _waiter,
			};
		
		let _server = Server {
				context : _context,
				state : _state,
				threads : _threads,
			};
		
		return Ok (_server);
	}
	
	
	pub fn wait (self) -> Outcome {
		
		self.threads.waiter.wait ();
		
		thread_join (self.threads.accepter_thread) .or_log_error (0x1f6f8582);
		thread_join (self.threads.handler_thread) .or_log_error (0xf02e2380);
		thread_join (self.threads.spawner_thread) .or_log_error (0x809d5832);
		thread_join (self.threads.ripper_thread) .or_log_error (0xc38eca69);
		
		return OK;
	}
}




fn server_accepter_loop (_self : ThreadState, _socket : socket2::Socket) -> Outcome {
	
	let mut _socket = _socket;
	
	log_debug! (0x73e7e20f, "server accepter started;");
	
	loop {
		
		if _self.should_stop () {
			break;
		}
		
		let _socket = match rpc_server_accept_once (&mut _socket, &_self.stopper) {
			Ok (Some (_socket)) =>
				_socket,
			Ok (None) =>
				break,
			Err (_error) => {
				_error.log_error (0x6b67fff8);
				break;
			}
		};
		
		let _client_thread = {
			let _thread_state = _self.clone ();
			thread_spawn ("z-exec.server.client", move || server_client_loop (_thread_state, _socket)) .or_panic (0x2f2a02f7)
		};
		
		// FIXME:  Do something with `_client_thread`!
	}
	
	log_debug! (0xeadd9ab2, "server accepter finished;");
	
	return OK;
}




fn server_client_loop (_self : ThreadState, _socket : socket2::Socket) -> Outcome {
	
	let mut _socket = _socket;
	let _handler_sender = _self.context.lock () .handler_sender.clone ();
	
	log_debug! (0x7d03d4b6, "server client started;");
	
	loop {
		
		if _self.should_stop () {
			break;
		}
		
		log_debug! (0x9c455e79, "server reading request...");
		
		let _request = match rpc_read_or_eof::<RpcRequestWrapper> (&mut _socket, None) {
			Ok (Some (_request)) =>
				_request,
			Ok (None) =>
				break,
			Err (_error) => {
				_error.log_error (0x38e36202);
				break;
			}
		};
		
		let _outcome = match _handler_sender.call (_request) {
			Ok (_outcome) =>
				_outcome,
			Err (_error) => {
				_error.log_error (0xe6641f26);
				break;
			}
		};
		
		match rpc_write (&mut _socket, &_outcome, None) {
			Ok (()) =>
				continue,
			Err (_error) => {
				_error.log_error (0xec4faf1e);
				break;
			}
		}
	}
	
	rpc_client_disconnect (_socket) .or_log_error (0x2c3b410d);
	
	log_debug! (0x2dfcf1f5, "server client finished;");
	
	return OK;
}




fn server_handler_loop (_self : ThreadState, _receiver : SyncCallReceiver<RpcRequestWrapper, RpcOutcomeBox>) -> Outcome {
	
	log_debug! (0x1dff9571, "server handler started;");
	let _spawner_sender = _self.context.lock () .spawner_sender.clone ();
	
	loop {
		
		if _self.should_stop () {
			break;
		}
		
		let (_request, _callback) = if let Some (_call) = _receiver.wait (time::Duration::from_millis (100)) {
			_call
		} else {
			continue;
		};
		
		let _response = match _request {
			
			RpcRequestWrapper::Execute (_request) =>
				match _spawner_sender.call (_request.descriptor) {
					Ok (Ok (mut _process)) => {
						let _identifier = _process.lock () .identifier.clone ();
						RpcOutcome::Ok (RpcExecuteResponse { identifier : _identifier }) .into_boxed ()
					},
					Ok (Err (_error)) =>
						RpcOutcomeBox::Err (_error.to_string ()),
					Err (_error) =>
						RpcOutcomeBox::Err (_error.to_string ()),
				}
		};
		
		_callback.done (_response);
	}
	
	log_debug! (0xe30fa44b, "server handler finished;");
	
	return OK;
}




fn server_spawner_loop (_self : ThreadState, _receiver : SyncCallReceiver<ProcessDescriptor, Outcome<SyncBox<ProcessState>>>) -> Outcome {
	
	log_debug! (0x081220f3, "server spawner started;");
	
	loop {
		
		if _self.should_stop () {
			break;
		}
		
		let (_descriptor, _callback) = if let Some (_call) = _receiver.wait (time::Duration::from_millis (100)) {
			_call
		} else {
			continue;
		};
		
		let _identifier = Identifier::new ();
		
		log_debug! (0xfea575bc, "server spawning process `{}`...", _identifier);
		
		// FIXME:  Make sure that nobody touches `spawn_std*` members!
		let _stdio = {
			let _context = _self.context.lock ();
			ProcessStdio {
					stdin : _context.spawn_stdin.as_ref () .map (io_unix::AsRawFd::as_raw_fd),
					stdout : _context.spawn_stdout.as_ref () .map (io_unix::AsRawFd::as_raw_fd),
					stderr : _context.spawn_stderr.as_ref () .map (io_unix::AsRawFd::as_raw_fd),
				}
		};
		
		let _pid = match process_spawn (&_descriptor, Some (env::vars_os ()), Some (&_stdio)) {
			Ok (_pid) =>
				_pid,
			Err (_error) => {
				_error.log_error (0x1ae5debe);
				_callback.failed (_error);
				continue;
			}
		};
		
		log_information! (0x9b169fe3, "server spawned process `{}` with pid `{}`;", _identifier, _pid);
		
		let _process = ProcessState {
				identifier : _identifier.clone (),
				pid : _pid,
				outcome : None,
			};
		
		let _process = SyncBox::new (_process);
		
		let mut _state = _self.state.lock ();
		_state.processes.insert (_identifier.clone (), _process.clone ()) .panic_if_some (0xb63f76e2);
		_state.processes_by_pid.insert (_pid, _identifier.clone ()) .panic_if_some (0x039ff515);
		
		_callback.succeeded (_process);
	}
	
	log_debug! (0x5dedfb34, "server spawner finished;");
	
	return OK;
}




fn server_ripper_loop (_self : ThreadState) -> Outcome {
	
	log_debug! (0x26f07c9f, "server ripper started;");
	
	loop {
		
		if _self.should_stop () {
			break;
		}
		
		let (_pid, _outcome) = match process_wait () {
			Ok (Some ((_pid, _outcome))) =>
				(_pid, _outcome),
			Ok (None) => {
				_self.sleep ();
				continue;
			},
			Err (_error) => {
				_error.log_error (0x4f904e87);
				_self.sleep ();
				continue;
			}
		};
		
		log_debug! (0x62720704, "server ripping process with pid `{}`...", _pid);
		
		let mut _state = _self.state.lock ();
		
		let _identifier = match _state.processes_by_pid.remove (&_pid) {
			Some (_identifier) =>
				_identifier,
			None => {
				log_warning! (0xb66deffa, "server ripped unknown process with pid `{}`;  ignoring!", _pid);
				continue;
			}
		};
		
		let _process = _state.processes.get (&_identifier) .or_panic (0xf858f41e);
		
		let mut _process = _process.lock ();
		_process.outcome = Some (_outcome);
		
		log_information! (0xd3e3e877, "server ripped process `{}` with pid `{}`;", _identifier, _pid);
	}
	
	log_debug! (0xf9a05bd1, "server ripper finished;");
	
	return OK;
}

