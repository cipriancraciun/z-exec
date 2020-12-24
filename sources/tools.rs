

use crate::prelude::*;




pub type Outcome<Value> = Result<Value, io::Error>;




pub(crate) fn log (_slug : &str, _level : u16, _code : u32, _message : impl fmt::Display) -> () {
	if (_level != 0) && (_level < DUMP_LOG_LEVEL) {
		return;
	}
	match (_slug, _code) {
		("", 0) =>
			eprintln! ("{}", _message),
		("", _) =>
			eprintln! ("[{:08x}]  {}", _code, _message),
		(_, 0) =>
			eprintln! ("{} {}", _slug, _message),
		(_, _) =>
			eprintln! ("{} [{:08x}]  {}", _slug, _code, _message),
	}
	unsafe {
		_log_empty = false;
		_log_cut_last = false;
	}
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

#[ allow (non_upper_case_globals) ]
static mut _log_empty : bool = true;
#[ allow (non_upper_case_globals) ]
static mut _log_cut_last : bool = false;




pub(crate) fn error (_code : u32, _message : impl fmt::Display) -> io::Error {
	
	let _message = format! ("[{:08x}]  {}", _code, _message);
	
	io::Error::new (io::ErrorKind::Other, _message)
}


pub(crate) fn error_wrap (_code : u32, _message : impl fmt::Display, _error : impl error::Error) -> io::Error {
	
	let _message = format! ("[{:08x}]  {}  //  {}", _code, _message, _error);
	
	io::Error::new (io::ErrorKind::Other, _message)
}




pub(crate) fn should_stop () -> sync::Arc<atomic::AtomicBool> {
	return SHOULD_STOP.clone ();
}


lazy_static::lazy_static! {
	static ref SHOULD_STOP : sync::Arc<atomic::AtomicBool> = {
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
			if let Err (_error) = signal_flag::register (_signal, _flag.clone ()) {
				log_error! (0x7c1c89e8, "unexpected error encountered;  ignoring!  //  {}", _error);
			}
		}
		_flag
	};
}

