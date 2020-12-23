

use crate::prelude::*;
use crate::lib::*;




pub struct WriteIoFmtAdapter <Write_ : io::Write> {
	stream : Write_,
	error : Option<io::Error>,
}


impl <Write_ : io::Write> WriteIoFmtAdapter<Write_> {
	
	pub fn new (_stream : Write_) -> Self {
		return Self {
				stream : _stream,
				error : None,
			};
	}
	
	pub fn finalize (self) -> Option<io::Error> {
		return self.error;
	}
}


impl <Write_ : io::Write> fmt::Write for WriteIoFmtAdapter<Write_> {
	
	fn write_str (&mut self, _string : &str) -> Result<(), fmt::Error> {
		if self.error.is_some () {
			return Err (fmt::Error);
		}
		match self.stream.write_all (_string.as_bytes ()) {
			Ok (()) =>
				return Ok (()),
			Err (_error) => {
				self.error = Some (_error);
				return Err (fmt::Error);
			}
		}
	}
}




pub struct WriteFmtIoAdapter <Write_ : fmt::Write> {
	stream : Write_,
	error : Option<fmt::Error>,
}


impl <Write_ : fmt::Write> WriteFmtIoAdapter<Write_> {
	
	pub fn new (_stream : Write_) -> Self {
		return Self {
				stream : _stream,
				error : None,
			};
	}
	
	pub fn finalize (self) -> Option<fmt::Error> {
		return self.error;
	}
}


impl <Write_ : fmt::Write> io::Write for WriteFmtIoAdapter<Write_> {
	
	fn write (&mut self, _buffer : &[u8]) -> Result<usize, io::Error> {
		if let Some (_error) = self.error {
			return Err (io::Error::new (io::ErrorKind::Other, _error));
		}
		match str::from_utf8 (_buffer) {
			Ok (_string) =>
				match self.stream.write_str (_string) {
					Ok (()) =>
						return Ok (_buffer.len ()),
					Err (_error) => {
						self.error = Some (_error);
						return Err (io::Error::new (io::ErrorKind::Other, _error));
					}
				},
			Err (_error) => {
				self.error = Some (fmt::Error);
				return Err (io::Error::new (io::ErrorKind::Other, _error));
			}
		}
	}
	
	fn flush (&mut self) -> Result<(), io::Error> {
		if let Some (_error) = self.error {
			return Err (io::Error::new (io::ErrorKind::Other, _error));
		}
		return Ok (());
	}
}

