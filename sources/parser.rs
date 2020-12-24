

use crate::prelude::*;
use crate::lib::*;




#[ derive (Debug) ]
#[ derive (Default) ]
pub struct ExecuteOptions {
	
	pub executable : OsString,
	pub argument0 : Option<OsString>,
	pub arguments : Vec<OsString>,
	
	pub environment_inherit : bool,
	pub environment_excludes : Vec<OsString>,
	pub environment_includes : Vec<OsString>,
	pub environment_overrides : Vec<OsString>,
}


impl ExecuteOptions {
	
	pub fn parser_prepare <'a, 'b : 'a> (&'b mut self, _parser : &mut argparse::ArgumentParser<'a>) -> () {
		
		_parser.refer (&mut self.executable) .metavar ("<executable>")
				.add_option (&["-x", "--exec", "--executable"], argparse::Store, "executable path (or name searched in PATH)")
				.required ();
		
		_parser.refer (&mut self.argument0) .metavar ("<argument0>")
				.add_option (&["--arg0", "--argument0"], argparse::StoreOption, "argument-0 (replaces executable in argument vector)");
		
		_parser.refer (&mut self.arguments) .metavar ("<arguments>")
				.add_argument ("argument", argparse::Collect, "plain argument");
		
		_parser.refer (&mut self.environment_inherit) .metavar ("<environment.inherit>")
				.add_option (&["--env-inherit"], argparse::StoreTrue, "environment inherit variables")
				.add_option (&["--env-clean", "--env-exclude-all"], argparse::StoreFalse, "environment inherit no variables");
		_parser.refer (&mut self.environment_excludes) .metavar ("<environment.exclude>")
				.add_option (&["--env-exclude"], argparse::Collect, "environment exclude variable with given name");
		_parser.refer (&mut self.environment_includes) .metavar ("<environment.include>")
				.add_option (&["--env-include"], argparse::Collect, "environment include variable with given name");
		_parser.refer (&mut self.environment_overrides) .metavar ("<environment.override>")
				.add_option (&["-e", "--env-set", "--env-override"], argparse::Collect, "environment override variable with given name and value;  <environment.override> := <key>=<value>");
	}
	
	
	pub fn descriptor_build (self) -> Outcome<ProcessDescriptor> {
		
		let ExecuteOptions {
				executable : _executable,
				argument0 : _argument0,
				arguments : _arguments,
				environment_inherit : _environment_inherit,
				environment_excludes : _environment_excludes,
				environment_includes : _environment_includes,
				environment_overrides : _environment_overrides,
			} = self;
		
		let _command = CommandDescriptor {
				executable : _executable.into (),
				argument0 : _argument0.map (StringDescriptor::from),
				arguments :
					if ! _arguments.is_empty () {
						Some (_arguments.into_iter () .map (StringDescriptor::from) .collect::<Vec<_>> () .into_boxed_slice () )
					} else { None },
			};
		
		let mut _environment_variables = Vec::with_capacity (_environment_excludes.len () + _environment_includes.len () + _environment_overrides.len ());
		for _key in _environment_excludes.into_iter () {
			if _key.as_bytes () .iter () .any (|&_byte| _byte == b'=') {
				fail! (0x1d71b18a, "invalid environment variable name (contains `=`): `{}`!", _key.to_string_lossy ());
			}
			_environment_variables.push (VariableDescriptor::Exclude { key : _key.into () });
		}
		for _key in _environment_includes.into_iter () {
			if _key.as_bytes () .iter () .any (|&_byte| _byte == b'=') {
				fail! (0x624ad9c3, "invalid environment variable name (contains `=`): `{}`!", _key.to_string_lossy ());
			}
			_environment_variables.push (VariableDescriptor::Include { key : _key.into () });
		}
		for _pair in _environment_overrides.into_iter () {
			let mut _pair = _pair.into_vec ();
			let (_key, _value) = if let Some (_index) = _pair.iter () .position (|&_byte| _byte == b'=') {
				let _value = _pair.split_off (_index + 1);
				_pair.pop ();
				(_pair, _value)
			} else {
				let _pair = OsString::from_vec (_pair);
				fail! (0xf56a54fb, "invalid environment variable pair (does not contain `=`): `{}`!", _pair.to_string_lossy ());
			};
			_environment_variables.push (VariableDescriptor::Override { key : _key.into (), value : _value.into () });
		}
		
		let _environment = EnvironmentDescriptor {
				inherit : _environment_inherit,
				variables :
						if ! _environment_variables.is_empty () {
							Some (_environment_variables.into_boxed_slice ())
						} else { None },
			};
		
		let _process = ProcessDescriptor {
				command : _command,
				environment : Some (_environment),
				directory : None,
				stdio : None,
			};
		
		return Ok (_process);
	}
}




#[ derive (Debug) ]
#[ derive (Default) ]
pub struct ServerListenOptions {
	pub unix_path : Option<PathBuf>,
	pub unix_path_remove : Option<bool>,
}


impl ServerListenOptions {
	
	pub fn parser_prepare <'a, 'b : 'a> (&'b mut self, _parser : &mut argparse::ArgumentParser<'a>) -> () {
		
		_parser.refer (&mut self.unix_path) .metavar ("<unix.path>")
				.add_option (&["-u", "--unix-path"], argparse::StoreOption, "UNIX domain socket path");
		_parser.refer (&mut self.unix_path_remove) .metavar ("<unix.path_remove>")
				.add_option (&["--unix-path-remove"], argparse::StoreConst (Some (true)), "remove UNIX domain socket path if it exists");
	}
}




#[ derive (Debug) ]
#[ derive (Default) ]
pub struct ServerHandleOptions {
	pub unix_fd : Option<u16>,
}


impl ServerHandleOptions {
	
	pub fn parser_prepare <'a, 'b : 'a> (&'b mut self, _parser : &mut argparse::ArgumentParser<'a>) -> () {
		
		_parser.refer (&mut self.unix_fd) .metavar ("<unix.path>")
				.add_option (&["--unix-fd"], argparse::StoreOption, "UNIX domain socket descriptor");
	}
}




#[ derive (Debug) ]
#[ derive (Default) ]
pub struct DumpOptions {
	
	pub rust : bool,
	pub ron : bool,
	pub json : bool,
}


impl DumpOptions {
	
	pub fn parser_prepare <'a, 'b : 'a> (&'b mut self, _parser : &mut argparse::ArgumentParser<'a>) -> () {
		
		_parser.refer (&mut self.rust)
				.add_option (&["--dump-rust"], argparse::StoreTrue, "dump process descriptor in Rust debug format");
		_parser.refer (&mut self.ron)
				.add_option (&["--dump-ron"], argparse::StoreTrue, "dump process descriptor in RON (Rusty Object Notation)");
		_parser.refer (&mut self.json)
				.add_option (&["--dump-json"], argparse::StoreTrue, "dump process descriptor in JSON format");
	}
	
	pub fn any (&self) -> bool {
		return self.rust || self.json || self.ron;
	}
	
	pub fn dump_rust (&self, _object : &impl fmt::Debug, _output : Option<&mut dyn io::Write>) -> Outcome<()> {
		if ! self.rust {
			return Ok (());
		}
		return self.dump_0 (
				|_output| {
					write! (_output, "{:#?}", _object) ?;
					write! (_output, "\n") ?;
					Ok (())
				},
				_output);
	}
	
	pub fn dump_ron (&self, _object : &impl SerializableRon, _output : Option<&mut dyn io::Write>) -> Outcome<()> {
		if ! self.ron {
			return Ok (());
		}
		return self.dump_0 (
				|mut _output| {
					_object.ron_into_stream (&mut _output, true) ?;
					write! (_output, "\n") ?;
					Ok (())
				},
				_output);
	}
	
	pub fn dump_json (&self, _object : &impl SerializableJson, _output : Option<&mut dyn io::Write>) -> Outcome<()> {
		if ! self.json {
			return Ok (());
		}
		return self.dump_0 (
				|mut _output| {
					_object.json_into_stream (&mut _output, true) ?;
					write! (_output, "\n") ?;
					Ok (())
				},
				_output);
	}
	
	fn dump_0 <Delegate> (&self, _delegate : Delegate, _output : Option<&mut dyn io::Write>) -> Outcome<()>
		where Delegate : FnOnce (&mut dyn io::Write) -> Outcome<()>
	{
		if let Some (_output) = _output {
			_delegate (_output) ?;
			_output.flush () ?;
		} else {
			let mut _output = io::stdout ();
			let mut _output = _output.lock ();
			_delegate (&mut _output) ?;
			_output.flush () ?;
		}
		return Ok (());
	}
}




pub fn parser_prepare <'a> () -> argparse::ArgumentParser<'a> {
	
	let mut _parser = argparse::ArgumentParser::new ();
	
	_parser.silence_double_dash (true);
	_parser.stop_on_first_argument (false);
	
	return _parser;
}


pub fn parser_execute (_parser : &argparse::ArgumentParser, _command : &str, _arguments : &[OsString]) -> Outcome<()> {
	
	let mut _arguments_0 = Vec::with_capacity (_arguments.len () + 1);
	_arguments_0.push (String::from (_command));
	for _argument in _arguments {
		match _argument.to_string_lossy () {
			Cow::Borrowed (_argument) =>
				_arguments_0.push (String::from (_argument)),
			Cow::Owned (_argument) =>
				fail! (0x42f4dade, "invalid argument (not UTF-8): `{}`!", _argument),
		}
	}
	
	let mut _stderr_1 = io::stderr ();
	let mut _stderr_2 = io::stderr ();
	
	match _parser.parse (_arguments_0, &mut _stderr_1, &mut _stderr_2) {
		Ok (()) =>
			(),
		Err (_code) =>
			fail! (0xdc46fc60, "exiting ({})!", _code),
	}
	
	_stderr_2.flush () ?;
	
	return Ok (());
}

