

use crate::prelude::*;




pub type Outcome<Value = ()> = Result<Value, Error>;

pub const OK : Outcome<()> = Ok (());




pub struct Error (io::Error);


impl error::Error for Error {}


impl fmt::Debug for Error {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		return self.0.fmt (_formatter);
	}
}


impl fmt::Display for Error {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		return self.0.fmt (_formatter);
	}
}




pub trait ResultExtWrap<Value> {
	
	fn or_wrap (self, _code : u32) -> Outcome<Value>;
}


impl <Value, Error : ErrorExtWrap> ResultExtWrap<Value> for Result<Value, Error> {
	
	fn or_wrap (self, _code : u32) -> Outcome<Value> {
		match self {
			Ok (_value) =>
				return Ok (_value),
			Err (_error) =>
				return Err (_error.wrap (_code)),
		}
	}
}




pub trait ErrorExtWrap {
	
	fn wrap (self, _code : u32) -> Error;
}


impl ErrorExtWrap for io::Error {
	
	fn wrap (self, _code : u32) -> Error {
		return error_wrap (_code, self);
	}
}




pub trait ResultExtLog<Value, Error : ErrorExtLog>
		where Self : Sized
{
	fn result (self) -> Result<Value, Error>;
	
	fn or_panic (self, _code : u32) -> Value {
		match self.result () {
			Ok (_value) =>
				return _value,
			Err (_error) =>
				_error.panic (_code),
		}
	}
	
	fn or_log_error (self, _code : u32) -> Option<Value> {
		match self.result () {
			Ok (_value) =>
				return Some (_value),
			Err (_error) => {
				_error.log_error (_code);
				return None;
			}
		}
	}
	
	fn or_log_warning (self, _code : u32) -> Option<Value> {
		match self.result () {
			Ok (_value) =>
				return Some (_value),
			Err (_error) => {
				_error.log_warning (_code);
				return None;
			}
		}
	}
}


impl <Value, Error : ErrorExtLog> ResultExtLog<Value, Error> for Result<Value, Error> {
	
	fn result (self) -> Self {
		return self;
	}
}




pub trait ErrorExtLog
		where Self : error::Error
{
	fn panic (&self, _code : u32) -> ! {
		panic_wrap! (0xe676e54a, self);
	}
	
	fn log_error (&self, _code : u32) -> () {
		log_error ("[ee]", LOG_LEVEL_ERROR, _code, self);
	}
	
	fn log_warning (&self, _code : u32) -> () {
		log_error ("[ww]", LOG_LEVEL_WARNING, _code, self);
	}
}


impl <Error : error::Error> ErrorExtLog for Error {}




pub(crate) fn log (_slug : &str, _level : u16, _code : u32, _message : impl fmt::Display) -> () {
	
	if (_level != 0) && (_level < DUMP_LOG_LEVEL) {
		return;
	}
	let _id = log_id ();
	match (_slug, _code) {
		("", 0) =>
			eprintln! ("{:15} {}", _id, _message),
		("", _) =>
			eprintln! ("{:15} [{:08x}]  {}", _id, _code, _message),
		(_, 0) =>
			eprintln! ("{:15} {} {}", _id, _slug, _message),
		(_, _) =>
			eprintln! ("{:15} {} [{:08x}]  {}", _id, _slug, _code, _message),
	}
	unsafe {
		_log_empty = false;
		_log_cut_last = false;
	}
}

pub(crate) fn log_error <Error : error::Error + ?Sized> (_slug : &str, _level : u16, _code : u32, _error : &Error) -> () {
	log (_slug, _level, _code, format_args! ("unexpected error encountered!  ignoring!  //  {}", _error));
}

pub(crate) fn log_cut (_forced : bool) -> () {
	if ! DUMP_LOG_CUT && ! _forced {
		return;
	}
	unsafe {
		if _log_cut_last {
			return;
		}
	}
	eprintln! ("[--]");
	unsafe {
		_log_empty = false;
		_log_cut_last = true;
	}
}

pub(crate) fn log_id () -> &'static str {
	thread_local! {
		static ID : cell::RefCell<String> = cell::RefCell::new (String::new ());
	}
	return ID.with (|_cell| {
			let _pid = nix::getpid ();
			let _tid = nix::gettid ();
			let _id = if _pid == _tid {
				format! ("[{}]", _pid)
			} else {
				format! ("[{}:{}]", _pid, _tid)
			};
			_cell.replace (_id);
			unsafe { &*_cell.as_ptr () } .as_str ()
		});
}

#[ allow (non_upper_case_globals) ]
static mut _log_empty : bool = true;
#[ allow (non_upper_case_globals) ]
static mut _log_cut_last : bool = false;




pub(crate) fn error (_code : u32) -> Error {
	
	let _message = format! ("[{:08x}]  unexpected error encountered!", _code);
	
	return Error (io::Error::new (io::ErrorKind::Other, _message))
}

pub(crate) fn error_with_message (_code : u32, _message : impl fmt::Display) -> Error {
	
	let _message = format! ("[{:08x}]  {}", _code, _message);
	
	return Error (io::Error::new (io::ErrorKind::Other, _message))
}


pub(crate) fn error_wrap (_code : u32, _error : impl error::Error) -> Error {
	
	let _message = format! ("[{:08x}]  unexpected error encountered!  //  {}", _code, _error);
	
	return Error (io::Error::new (io::ErrorKind::Other, _message))
}

pub(crate) fn error_wrap_with_message (_code : u32, _message : impl fmt::Display, _error : impl error::Error) -> Error {
	
	let _message = format! ("[{:08x}]  {}  //  {}", _code, _message, _error);
	
	return Error (io::Error::new (io::ErrorKind::Other, _message))
}




pub(crate) fn should_stop () -> SyncTrigger {
	return SHOULD_STOP.clone ();
}


lazy_static::lazy_static! {
	static ref SHOULD_STOP : SyncTrigger = {
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
			signal_flag::register (_signal, _flag.clone ()) .or_log_error (0xf970b50e);
		}
		SyncTrigger (_flag)
	};
}




pub struct SyncTrigger (sync::Arc<atomic::AtomicBool>);


impl SyncTrigger {
	
	pub fn new () -> Self {
		return SyncTrigger (sync::Arc::new (atomic::AtomicBool::new (false)));
	}
	
	pub fn clone (&self) -> Self {
		return SyncTrigger (self.0.clone ());
	}
	
	pub fn trigger (&self) -> () {
		self.0.store (true, atomic::Ordering::Relaxed);
	}
	
	pub fn is_triggered (&self) -> bool {
		return self.0.load (atomic::Ordering::Relaxed);
	}
}




pub struct SyncBox <Value> (sync::Arc<sync::Mutex<cell::Cell<Value>>>);

pub struct SyncBoxRef <'a, Value> (owning_ref::OwningRefMut<sync::MutexGuard<'a, cell::Cell<Value>>, Value>);


impl <Value> SyncBox<Value> {
	
	pub fn new (_value : Value) -> Self {
		return SyncBox (sync::Arc::new (sync::Mutex::new (cell::Cell::new (_value))));
	}
	
	pub fn clone (&self) -> Self {
		return SyncBox (sync::Arc::clone (&self.0));
	}
	
	pub fn lock (&self) -> SyncBoxRef<Value> {
		let _lock = match self.0.lock () {
			Ok (_lock) =>
				_lock,
			Err (_) =>
				panic_assertion! (0xfd80f2ca),
		};
		return SyncBoxRef (owning_ref::MutexGuardRefMut::new (_lock) .map_mut (cell::Cell::get_mut));
	}
}


impl <'a, Value> Deref for SyncBoxRef <'a, Value> {
	
	type Target = Value;
	
	fn deref (&self) -> &Value {
		return self.0.deref ();
	}
}


impl <'a, Value> DerefMut for SyncBoxRef <'a, Value> {
	
	fn deref_mut (&mut self) -> &mut Value {
		return self.0.deref_mut ();
	}
}




pub struct SyncCallSender <Input, Output> {
	invoke_sender : mpsc::SyncSender<(Input, mpsc::SyncSender<Output>)>,
}

pub struct SyncCallReceiver <Input, Output> {
	invoke_receiver : mpsc::Receiver<(Input, mpsc::SyncSender<Output>)>,
}

pub struct SyncCallReturn <Output> {
	return_sender : mpsc::SyncSender<Output>,
}


pub fn sync_call_new <Input, Output> () -> (SyncCallSender<Input, Output>, SyncCallReceiver<Input, Output>) {
	let (_invoke_sender, _invoke_receiver) = mpsc::sync_channel (0);
	return (
			SyncCallSender {
					invoke_sender : _invoke_sender,
				},
			SyncCallReceiver {
					invoke_receiver : _invoke_receiver,
				},
		);
}


impl <Input, Output> SyncCallSender<Input, Output> {
	
	pub fn clone (&self) -> Self {
		return SyncCallSender {
				invoke_sender : self.invoke_sender.clone ()
			};
	}
	
	pub fn call (&self, _input : Input) -> Outcome<Output> {
		let (_return_sender, _return_receiver) = mpsc::sync_channel (0);
		match self.invoke_sender.send ((_input, _return_sender)) {
			Ok (()) =>
				(),
			Err (_) =>
				fail! (0x48ffcef9, "handler terminated!"),
		}
		match _return_receiver.recv () {
			Ok (_output) =>
				return Ok (_output),
			Err (_) =>
				fail! (0x3ec9dae6, "handler terminated!"),
		}
	}
}


impl <Input, Output> SyncCallReceiver<Input, Output> {
	
	pub fn wait (&self, _timeout : time::Duration) -> Option<(Input, SyncCallReturn<Output>)> {
		match self.invoke_receiver.recv_timeout (_timeout) {
			Ok ((_input, _return_sender)) => {
				let _return = SyncCallReturn {
						return_sender : _return_sender,
					};
				return Some ((_input, _return));
			}
			Err (mpsc::RecvTimeoutError::Timeout) =>
				return None,
			Err (mpsc::RecvTimeoutError::Disconnected) =>
				return None,
		}
	}
}


impl <Output> SyncCallReturn<Output> {
	
	pub fn done (self, _output : Output) -> () {
		match self.return_sender.send (_output) {
			Ok (()) =>
				(),
			Err (_) =>
				log_warning! (0x2668c2f3, "caller terminated;  ignoring!"),
		}
	}
}


impl <Value, Error> SyncCallReturn<Result<Value, Error>> {
	
	pub fn succeeded (self, _value : Value) -> () {
		self.done (Ok (_value));
	}
	
	pub fn failed (self, _error : Error) -> () {
		self.done (Err (_error));
	}
}




pub fn thread_spawn <Delegate> (_name : &str, _delegate : Delegate) -> Outcome<thread::JoinHandle<Outcome<()>>>
		where Delegate : FnOnce () -> Outcome<()> + Send + 'static
{
	let _builder = thread::Builder::new () .name (String::from (_name));
	let _joiner = _builder.spawn (_delegate) .or_wrap (0x33fb9a81) ?;
	return Ok (_joiner);
}

pub fn thread_join (_joiner : thread::JoinHandle<Outcome<()>>) -> Outcome<()> {
	match _joiner.join () {
		Ok (_outcome) =>
			return _outcome,
		Err (_error) =>
			fail! (0xa26812e1, "thread terminated! //  {:?}", _error),
	}
}




pub trait OptionExt <Value>
		where Self : Sized
{
	
	fn option (self) -> Option<Value>;
	
	fn or_panic (self, _code : u32) -> Value {
		match self.option () {
			Some (_value) =>
				return _value,
			None =>
				panic_with_message! (_code, "unexpected option missing value!"),
		}
	}
	
	fn panic_if_some (self, _code : u32) -> () {
		self.panic_if (_code, false);
	}
	
	fn panic_if_none (self, _code : u32) -> () {
		self.panic_if (_code, true);
	}
	
	fn panic_if (self, _code : u32, _expected : bool) -> () {
		match self.option () {
			Some (_) =>
				if ! _expected {
					panic_with_message! (_code, "unexpected option present value!");
				}
			None =>
				if _expected {
					panic_with_message! (_code, "unexpected option missing value!");
				}
		}
	}
}


impl <Value> OptionExt<Value> for Option<Value> {
	
	fn option (self) -> Self {
		return self;
	}
}

