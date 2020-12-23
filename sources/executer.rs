

use crate::prelude::*;
use crate::lib::*;




pub fn execute (_descriptor : &ProcessDescriptor, _environment : Option<impl Iterator<Item = (OsString, OsString)>>) -> Outcome<()> {
	
	let _command = &_descriptor.command;
	
//	close_all_fd () ?;
	
	let _executable = convert_osstr_to_cstring (&_command.executable.as_ref ()) ?;
	let _arguments = build_arguments (&_descriptor.command) ?;
	let _environment = build_environment (_descriptor.environment.as_ref (), _environment) ?;
	
	match nix::execvpe (&_executable, &_arguments, &_environment) {
		Ok (_) =>
			fail_assertion! (0x2ae4ad32),
		Err (_error) =>
			fail_wrap! (0x12f9517e, "failed calling `execvpe`!", _error),
	}
}




pub fn build_arguments (_descriptor : &CommandDescriptor) -> Outcome<Vec<CString>> {
	
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




pub fn build_environment (_descriptor : Option<&EnvironmentDescriptor>, _inherited : Option<impl Iterator<Item = (OsString, OsString)>>) -> Outcome<Vec<CString>> {
	
	static DESCRIPTOR_FALLBACK : EnvironmentDescriptor = EnvironmentDescriptor {
			inherit : true,
			variables : None,
		};
	
	let _descriptor = _descriptor.unwrap_or (&DESCRIPTOR_FALLBACK);
	
	let _inherited = if let Some (_inherited) = _inherited {
		let mut _map = HashMap::with_capacity (1024);
		for (_key, _value) in _inherited {
			if _key.as_bytes () .iter () .any (|&_byte| _byte == b'=') {
				fail! (0xa67f0db6, "invalid environment (key contains `=`): `{:?}`!", _key);
			}
			if let Some (_value) = _map.insert (_key.clone (), _value) {
				fail! (0x6f0969c8, "invalid environment (duplicate key): `{:?}`!", _key);
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
				fail! (0x28e0c9c8, "invalid environment (key contains `=`): `{:?}`!", _key);
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




pub fn close_all_fd () -> Outcome<()> {
	
	let _limit : io_unix::RawFd = unsafe {
		let _limit = libc::getdtablesize ();
		if _limit > 0 {
			_limit
		} else {
			fail! (0x3bc3a0d2, "failed calling `getdtablesize`!");
		}
	};
	
	for _descriptor in 0 .. _limit {
		match nix::close (_descriptor) {
			Ok (()) =>
				// NOTE:  Descriptor existed and was closed.
				(),
			Err (nix::Error::Sys (nix::Errno::EBADF)) =>
				// NOTE:  Descriptor did not exist and was ignored.
				(),
			Err (_error) =>
				fail_wrap! (0x45bcdf29, "failed calling `close`!", _error),
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
			fail! (0x52417275, "invalid string (contains `\0`): `{:?}`!", _value),
	}
}

