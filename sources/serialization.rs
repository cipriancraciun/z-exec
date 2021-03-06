

use crate::prelude::*;
use crate::lib::*;




pub fn serialize_json_into_string <Value : serde::Serialize + ?Sized> (_value : &Value, _string : &mut String, _pretty : bool) -> Outcome<()> {
	let mut _stream = WriteFmtIoAdapter::new (_string);
	let _outcome = serialize_json_into_stream (_value, &mut _stream, _pretty);
	if let Some (_error) = _stream.finalize () {
		fail_wrap! (0x65dbef3a, "failed serialization!"; _error);
	}
	match _outcome {
		Ok (()) =>
			return Ok (()),
		Err (_error) =>
			fail_wrap! (0xd9e66e38, "failed serialization!"; _error),
	}
}

pub fn serialize_json_into_stream <Value : serde::Serialize + ?Sized, Stream : io::Write + ?Sized> (_value : &Value, _stream : &mut Stream, _pretty : bool) -> Outcome<()> {
	let _outcome = if _pretty {
		let mut _serializer = serde_json::ser::Serializer::pretty (_stream);
		_value.serialize (&mut _serializer)
	} else {
		let mut _serializer = serde_json::ser::Serializer::new (_stream);
		_value.serialize (&mut _serializer)
	};
	match _outcome {
		Ok (()) =>
			return Ok (()),
		Err (_error) =>
			fail_wrap! (0x529f5ea9, "failed serialization!"; _error),
	}
}


pub fn deserialize_json_from_string <Value : serde::de::DeserializeOwned>  (_string : &str) -> Outcome<Value> {
	match serde_json::de::from_str (_string) {
		Ok (_value) =>
			return Ok (_value),
		Err (_error) =>
			fail_wrap! (0xa4bf801b, "failed deserialization!"; _error),
	}
}

pub fn deserialize_json_from_stream <Value : serde::de::DeserializeOwned, Stream : io::Read + ?Sized> (_stream : &mut Stream) -> Outcome<Value> {
	match serde_json::de::from_reader (_stream) {
		Ok (_value) =>
			return Ok (_value),
		Err (_error) =>
			fail_wrap! (0x2bd3b5d9, "failed deserialization!"; _error),
	}
}




pub trait SerializableJson : serde::Serialize {
	
	fn json_into_string (&self, _string : &mut String, _pretty : bool) -> Outcome<()> {
		return serialize_json_into_string (self, _string, _pretty);
	}
	
	fn json_into_stream <Stream : io::Write + ?Sized> (&self, _stream : &mut Stream, _pretty : bool) -> Outcome<()> {
		return serialize_json_into_stream (self, _stream, _pretty);
	}
}


pub trait DeserializableJson : serde::de::DeserializeOwned {
	
	fn json_from_string (_string : &str) -> Outcome<Self> {
		return deserialize_json_from_string (_string);
	}
	
	fn json_from_stream <Stream : io::Read + ?Sized> (_stream : &mut Stream) -> Outcome<Self> {
		return deserialize_json_from_stream (_stream);
	}
}




pub fn serialize_ron_into_string <Value : serde::Serialize + ?Sized> (_value : &Value, _string : &mut String, _pretty : bool) -> Outcome<()> {
	let mut _stream = WriteFmtIoAdapter::new (_string);
	let _outcome = serialize_ron_into_stream (_value, &mut _stream, _pretty);
	if let Some (_error) = _stream.finalize () {
		fail_wrap! (0x3a6aa74c, "failed serialization!"; _error);
	}
	match _outcome {
		Ok (()) =>
			return Ok (()),
		Err (_error) =>
			fail_wrap! (0x2f6a2251, "failed serialization!"; _error),
	}
}

pub fn serialize_ron_into_stream <Value : serde::Serialize + ?Sized, Stream : io::Write + ?Sized> (_value : &Value, _stream : &mut Stream, _pretty : bool) -> Outcome<()> {
	let _configuration = if _pretty {
		let _configuration = serde_ron::ser::PrettyConfig::new ()
				.with_separate_tuple_members (true)
				.with_indentor (String::from ("  "))
			;
		Some (_configuration)
	} else {
		None
	};
	let mut _serializer = match serde_ron::ser::Serializer::new (_stream, _configuration, false) {
		Ok (_serializer) =>
			_serializer,
		Err (_error) =>
			fail_wrap! (0x1d67fd96, "failed serialization!"; _error),
	};
	match _value.serialize (&mut _serializer) {
		Ok (()) =>
			return Ok (()),
		Err (_error) =>
			fail_wrap! (0xeed758c1, "failed serialization!"; _error),
	}
}


pub fn deserialize_ron_from_string <Value : serde::de::DeserializeOwned> (_string : &str) -> Outcome<Value> {
	match serde_ron::de::from_str (_string) {
		Ok (_value) =>
			return Ok (_value),
		Err (_error) =>
			fail_wrap! (0x139c3281, "failed deserialization!"; _error),
	}
}

pub fn deserialize_ron_from_stream <Value : serde::de::DeserializeOwned, Stream : io::Read + ?Sized> (_stream : &mut Stream) -> Outcome<Value> {
	match serde_ron::de::from_reader (_stream) {
		Ok (_value) =>
			return Ok (_value),
		Err (_error) =>
			fail_wrap! (0x8dc2b764, "failed deserialization!"; _error),
	}
}




pub trait SerializableRon : serde::Serialize {
	
	fn ron_into_string (&self, _string : &mut String, _pretty : bool) -> Outcome<()> {
		return serialize_ron_into_string (self, _string, _pretty);
	}
	
	fn ron_into_stream <Stream : io::Write + ?Sized> (&self, _stream : &mut Stream, _pretty : bool) -> Outcome<()> {
		return serialize_ron_into_stream (self, _stream, _pretty);
	}
}


pub trait DeserializableRon : serde::de::DeserializeOwned {
	
	fn ron_from_string (_string : &str) -> Outcome<Self> {
		return deserialize_ron_from_string (_string);
	}
	
	fn ron_from_stream <Stream : io::Read + ?Sized> (_stream : &mut Stream) -> Outcome<Self> {
		return deserialize_ron_from_stream (_stream);
	}
}




pub trait SerializableRaw : serde::Serialize + serde_erased::Serialize {}
pub trait DeserializableRaw : serde::de::DeserializeOwned {}

pub trait SerializableErased : serde_erased::Serialize {}




pub trait Serializable : SerializableJson + DeserializableJson + SerializableRon + DeserializableRon + SerializableRaw + DeserializableRaw + SerializableErased {}
pub trait SerializableOnly : SerializableJson + SerializableRon + SerializableRaw {}
pub trait DeserializableOnly : DeserializableJson + DeserializableRon + DeserializableRaw {}




serializable! (Box<Value> : SerializableOnly);

