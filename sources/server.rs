

use crate::prelude::*;
use crate::lib::*;




pub struct Server {
	
	context : SyncBox<ServerContext>,
	state : SyncBox<ServerState>,
	threads : ServerThreads,
}


struct ServerState {
	
}


pub struct ServerContext {
	
	handler_sender : SyncCallSender<RpcRequestWrapper, RpcOutcomeBox>,
	spawner_sender : SyncCallSender<(), Outcome<()>>,
}


pub struct ServerThreads {
	
	accepter_thread : thread::JoinHandle<Outcome>,
	handler_thread : thread::JoinHandle<Outcome>,
	spawner_thread : thread::JoinHandle<Outcome>,
	ripper_thread : thread::JoinHandle<Outcome>,
	
	stopper : SyncTrigger,
	waiter : crossbeam_sync::WaitGroup,
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
		
		let _waiter = crossbeam_sync::WaitGroup::new ();
		
		let (_handler_sender, _handler_receiver) = sync_call_new ();
		let (_spawner_sender, _spawner_receiver) = sync_call_new ();
		
		let _context = ServerContext {
				handler_sender : _handler_sender,
				spawner_sender : _spawner_sender,
			};
		
		let _state = ServerState {};
		
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
	
	loop {
		
		if _self.should_stop () {
			break;
		}
		
		let _call = if let Some (_call) = _receiver.wait (time::Duration::from_millis (100)) {
			_call
		} else {
			continue;
		};
		
		let _request = &_call.input;
		
		let _response = match _request {
			
			RpcRequestWrapper::Execute (_request) =>
				match spawn (&_request.descriptor, Some (env::vars_os ())) {
					Ok (_pid) =>
						RpcOutcome::Ok (RpcExecuteResponse { pid : _pid }) .into_boxed (),
					Err (_error) =>
						RpcOutcomeBox::Err (_error.to_string ()),
				}
		};
		
		_call.done (_response);
	}
	
	log_debug! (0xe30fa44b, "server handler finished;");
	
	return OK;
}




fn server_spawner_loop (_self : ThreadState, _receiver : SyncCallReceiver<(), Outcome<()>>) -> Outcome {
	
	log_debug! (0x081220f3, "server spawner started;");
	
	loop {
		
		if _self.should_stop () {
			break;
		}
		
		_self.sleep ();
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
		
		_self.sleep ();
	}
	
	log_debug! (0xf9a05bd1, "server ripper finished;");
	
	return OK;
}

