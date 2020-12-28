

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
	
	accepter_socket : socket2::Socket,
	
	spawner_sender : SyncCallSender<(), Outcome<()>>,
	
	should_stop : SyncTrigger,
}


pub struct ServerThreads {
	waiter : crossbeam_sync::WaitGroup,
	accepter_thread : thread::JoinHandle<Outcome<()>>,
	spawner_thread : thread::JoinHandle<Outcome<()>>,
	ripper_thread : thread::JoinHandle<Outcome<()>>,
}




impl Server {
	
	pub fn start (_socket : socket2::Socket, _should_stop : SyncTrigger) -> Outcome<Server> {
		
		let _waiter = crossbeam_sync::WaitGroup::new ();
		
		let (_spawner_sender, _spawner_receiver) = sync_call_new ();
		
		let _context = ServerContext {
				accepter_socket : _socket,
				spawner_sender : _spawner_sender,
				should_stop : _should_stop,
			};
		
		let _state = ServerState {};
		
		let _context = SyncBox::new (_context);
		let _state = SyncBox::new (_state);
		
		let _accepter_thread = {
			let _context = _context.clone ();
			let _state = _state.clone ();
			let _waiter = _waiter.clone ();
			thread_spawn ("z-exec.server.accepter", move || server_accepter_loop (_context, _state, _waiter)) .or_panic (0x6d515f7e)
		};
		
		let _spawner_thread = {
			let _context = _context.clone ();
			let _state = _state.clone ();
			let _waiter = _waiter.clone ();
			thread_spawn ("z-exec.server.spawner", move || Ok (())) .or_panic (0xea492bcc)
		};
		
		let _ripper_thread = {
			let _context = _context.clone ();
			let _state = _state.clone ();
			let _waiter = _waiter.clone ();
			thread_spawn ("z-exec.server.ripper", move || Ok (())) .or_panic (0x37277e58)
		};
		
		let _threads = ServerThreads {
				accepter_thread : _accepter_thread,
				spawner_thread : _spawner_thread,
				ripper_thread : _ripper_thread,
				waiter : _waiter,
			};
		
		let _server = Server {
				context : _context,
				state : _state,
				threads : _threads,
			};
		
		return Ok (_server);
	}
}




fn server_accepter_loop (_context : SyncBox<ServerContext>, _state : SyncBox<ServerState>, _waiter : crossbeam_sync::WaitGroup) -> Outcome<()> {
	
	let _context_lock = _context.lock ();
	let mut _socket = _context_lock.accepter_socket.try_clone () .or_panic (0x3521a15d);
	drop (_context_lock);
	
	let _socket = &mut _socket;
	
	return Ok (());
}


fn server_spawner_loop (_context : SyncBox<ServerContext>, _state : SyncBox<ServerState>, _waiter : crossbeam_sync::WaitGroup) -> Outcome<()> {
	
	return Ok (());
}


fn server_ripper_loop (_context : SyncBox<ServerContext>, _state : SyncBox<ServerState>, _waiter : crossbeam_sync::WaitGroup) -> Outcome<()> {
	
	return Ok (());
}

