

use crate::prelude::*;
use crate::lib::*;




pub struct ProcessStdio {
	pub stdin : Option<io_unix::RawFd>,
	pub stdout : Option<io_unix::RawFd>,
	pub stderr : Option<io_unix::RawFd>,
}




pub fn process_execute (_descriptor : &ProcessDescriptor, _environment : Option<impl Iterator<Item = (OsString, OsString)>>, _stdio : Option<&ProcessStdio>) -> Outcome<()> {
	
	let _command = &_descriptor.command;
	
	if let Some (_stdio) = _stdio {
		process_fd_stdio (_stdio.stdin, _stdio.stdout, _stdio.stderr) ?;
	}
	process_fd_close_all (true, true, true) ?;
	
	let _executable = convert_osstr_to_cstring (&_command.executable.as_ref ()) ?;
	let _arguments = process_build_arguments (&_descriptor.command) ?;
	let _environment = process_build_environment (_descriptor.environment.as_ref (), _environment) ?;
	
	match nix::execvpe (&_executable, &_arguments, &_environment) {
		Ok (_) =>
			fail_assertion! (0x2ae4ad32),
		Err (_error) =>
			fail_wrap! (0x12f9517e, "failed calling `execvpe`!"; _error),
	}
}




pub fn process_spawn (_descriptor : &ProcessDescriptor, _environment : Option<impl Iterator<Item = (OsString, OsString)>>, _stdio : Option<&ProcessStdio>) -> Outcome<ProcessId> {
	
	match unsafe { nix::fork () } {
		
		Ok (nix::ForkResult::Parent { child : _child, .. }) =>
			return Ok (ProcessId::from_raw (_child.as_raw ())),
		
		Ok (nix::ForkResult::Child) => {
			match process_execute (_descriptor, _environment, _stdio) {
				Ok (_) =>
					fail_assertion! (0x32933043),
				Err (_error) => {
					log_error! (0xdad78bb5, "unexpected error encountered;  aborting!  //  {}", _error);
					process::exit (1);
				}
			}
		}
		
		Err (_error) =>
			fail_wrap! (0x16900d78, "failed fork!"; _error),
	}
}




pub fn process_wait () -> Outcome<Option<(ProcessId, ProcessOutcome)>> {
	
	match nix::waitpid (None, Some (nix::WaitPidFlag::WNOHANG)) {
		
		Ok (nix::WaitStatus::Exited (_id, _exit)) =>
			return Ok (Some ((ProcessId::from_raw (_id.as_raw ()), ProcessOutcome::Exited (ProcessExit::from_raw (_exit))))),
		Ok (nix::WaitStatus::Signaled (_id, _signal, _)) =>
			return Ok (Some ((ProcessId::from_raw (_id.as_raw ()), ProcessOutcome::Killed (ProcessSignal::from_raw (_signal as libc::c_int))))),
		
		Ok (_) =>
			return Ok (None),
		
		Err (nix::Error::Sys (nix::ECHILD)) =>
			return Ok (None),
		
		Err (_error) =>
			fail_wrap! (0x09fa400d, "failed wait!"; _error),
	}
}




pub fn process_build_arguments (_descriptor : &CommandDescriptor) -> Outcome<Vec<CString>> {
	
	let _argument0 = if let Some (_argument0) = &_descriptor.argument0 {
		convert_osstr_to_cstring (_argument0) ?
	} else {
		convert_osstr_to_cstring (&_descriptor.executable) ?
	};
	
	let mut _arguments : Vec<CString> = Vec::with_capacity (1 + _descriptor.arguments.as_ref () .map_or (0, |_arguments| _arguments.len ()));
	_arguments.push (_argument0);
	if let Some (_descriptor) = _descriptor.arguments.as_ref () {
		for _argument in _descriptor.iter () {
			let _argument = convert_osstr_to_cstring (_argument) ?;
			_arguments.push (_argument);
		}
	}
	
	return Ok (_arguments);
}




pub fn process_build_environment (_descriptor : Option<&EnvironmentDescriptor>, _inherited : Option<impl Iterator<Item = (OsString, OsString)>>) -> Outcome<Vec<CString>> {
	
	static DESCRIPTOR_FALLBACK : EnvironmentDescriptor = EnvironmentDescriptor {
			inherit : true,
			variables : None,
		};
	
	let _descriptor = _descriptor.unwrap_or (&DESCRIPTOR_FALLBACK);
	
	let _inherited = if let Some (_inherited) = _inherited {
		let mut _map = HashMap::with_capacity (1024);
		for (_key, _value) in _inherited {
			if _key.as_bytes () .iter () .any (|&_byte| _byte == b'=') {
				fail! (0xa67f0db6, "invalid environment (key contains `=`): `{}`!", _key.to_string_lossy ());
			}
			if let Some (_value) = _map.insert (_key.clone (), _value) {
				fail! (0x6f0969c8, "invalid environment (duplicate key): `{}`!", _key.to_string_lossy ());
			}
		}
		_map
	} else {
		HashMap::new ()
	};
	
	let mut _environment = if _descriptor.inherit {
		_inherited.clone ()
	} else {
		HashMap::with_capacity (_descriptor.variables.as_ref () .map_or (0, |_variables| _variables.len ()))
	};
	
	if let Some (_descriptors) = &_descriptor.variables {
		for _descriptor in _descriptors.iter () {
			match _descriptor {
				VariableDescriptor::Include { key : _key } => {
					if let Some (_value) = _inherited.get (_key.as_ref ()) {
						_environment.insert (_key.into (), _value.clone ());
					}
				}
				VariableDescriptor::Exclude { key : _key } => {
					_environment.remove (_key.as_ref ());
				}
				VariableDescriptor::Override { key : _key, value : _value } => {
					_environment.insert (_key.into (), _value.into ());
				}
			}
		}
	}
	
	let _environment = {
		let mut _pairs = Vec::with_capacity (_environment.len ());
		for (_key, _value) in _environment.into_iter () {
			if _key.as_bytes () .iter () .any (|&_byte| _byte == b'=') {
				fail! (0x28e0c9c8, "invalid environment (key contains `=`): `{}`!", _key.to_string_lossy ());
			}
			let mut _key = _key.into_vec ();
			let mut _value = _value.into_vec ();
			let mut _pair = Vec::with_capacity (_key.len () + 1 + _value.len ());
			_pair.append (&mut _key);
			_pair.push (b'=');
			_pair.append (&mut _value);
			let _pair = OsString::from_vec (_pair);
			let _pair = convert_osstring_to_cstring (_pair) ?;
			_pairs.push (_pair);
		}
		_pairs.sort ();
		_pairs
	};
	
	return Ok (_environment);
}




pub fn process_fd_stdio (_stdin : Option<io_unix::RawFd>, _stdout : Option<io_unix::RawFd>, _stderr : Option<io_unix::RawFd>) -> Outcome<()> {
	
	if let Some (_stdin) = _stdin {
		if _stdin != 0 {
			log_debug! (0x79cff2f5, "setting stdin to `{}`...", _stdin);
			nix::dup2 (_stdin, 0) .or_wrap (0xcbdbaef8) ?;
		}
	}
	if let Some (_stdout) = _stdout {
		if _stdout != 1 {
			log_debug! (0x79cff2f5, "setting stdout to `{}`...", _stdout);
			nix::dup2 (_stdout, 1) .or_wrap (0x4577385e) ?;
		}
	}
	if let Some (_stderr) = _stderr {
		if _stderr != 2 {
			log_debug! (0x79cff2f5, "setting stderr to `{}`...", _stderr);
			nix::dup2 (_stderr, 2) .or_wrap (0x1dbf5a04) ?;
		}
	}
	
	return Ok (());
}




pub fn process_fd_close_all (_skip_stdin : bool, _skip_stdout : bool, _skip_stderr : bool) -> Outcome<()> {
	
	let _limit : io_unix::RawFd = unsafe {
		let _limit = libc::getdtablesize ();
		if _limit > 0 {
			_limit
		} else {
			fail! (0x3bc3a0d2, "failed calling `getdtablesize`!");
		}
	};
	
	for _descriptor in 0 .. _limit {
		
		match _descriptor {
			0 if _skip_stdin => continue,
			1 if _skip_stdout => continue,
			2 if _skip_stderr => continue,
			_ => (),
		}
		
		let _path = PathBuf::from (format! ("/proc/self/fd/{}", _descriptor));
		match nix::readlink (&_path) {
			Ok (_path) =>
				log_debug! (0xa471f1e8, "closing file `{}` pointing to `{}`...", _descriptor, _path.to_string_lossy ()),
			Err (nix::Error::Sys (nix::ENOENT)) =>
				(),
			Err (_error) =>
				log_error! (0x19d71755, "failed calling `readlink` for `{}`;  ignoring!  //  {}", _descriptor, _error),
		}
		
		match nix::close (_descriptor) {
			Ok (()) =>
				log_debug! (0x011f2af2, "closed file `{}`;", _descriptor),
			Err (nix::Error::Sys (nix::EBADF)) =>
				(),
			Err (_error) =>
				log_error! (0x442896d3, "failed calling `close` for `{}`;  ignoring!  //  {}", _descriptor, _error),
		}
	}
	
	return Ok (());
}




pub fn convert_osstr_to_cstring (_value : & impl AsRef<OsStr>) -> Outcome<CString> {
	let _value = _value.as_ref () .to_os_string ();
	return convert_osstring_to_cstring (_value);
}

pub fn convert_osstring_to_cstring (_value : OsString) -> Outcome<CString> {
	match CString::new (_value.clone () .into_vec ()) {
		Ok (_value) =>
			Ok (_value),
		Err (_) =>
			fail! (0x52417275, "invalid string (contains `\0`): `{}`!", _value.to_string_lossy ()),
	}
}

