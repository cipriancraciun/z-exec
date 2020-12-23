

use crate::prelude::*;
use crate::lib::*;




#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct ProcessDescriptor {
	
	pub command : CommandDescriptor,
	
	#[ serde (skip_serializing) ]
	pub environment : Option<EnvironmentDescriptor>,
	
	#[ serde (skip_serializing) ]
	pub directory : Option<StringDescriptor>,
	
	#[ serde (skip_serializing) ]
	pub stdio : Option<StdioDescriptor>,
}


#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct CommandDescriptor {
	
	pub executable : StringDescriptor,
	
	#[ serde (skip_serializing) ]
	pub argument0 : Option<StringDescriptor>,
	
	#[ serde (skip_serializing) ]
	pub arguments : Option<Box<[StringDescriptor]>>,
}


#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct EnvironmentDescriptor {
	
	pub inherit : bool,
	
	#[ serde (skip_serializing) ]
	pub variables : Option<Box<[VariableDescriptor]>>,
}


#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub enum VariableDescriptor {
	Include (StringDescriptor),
	Exclude (StringDescriptor),
	Override (StringDescriptor, StringDescriptor),
}

#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct StdioDescriptor {
	pub stdin : IoDescriptor,
	pub stdout : IoDescriptor,
	pub stderr : IoDescriptor,
}


#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub enum IoDescriptor {
	Inherit,
	DevNull,
}




#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct StringDescriptor (
	
	// FIXME:  Add support for non UTF-8 strings!
	// #[ serde (with = "serde_bytes") ]
	// Box<[u8]>,
	
	Box<str>,
);

impl AsRef<OsStr> for StringDescriptor {
	fn as_ref (&self) -> &OsStr {
		// return OsStr::from_bytes (self.0.as_ref ());
		return OsStr::new (self.0.as_ref ());
	}
}

impl Into<OsString> for StringDescriptor {
	fn into (self) -> OsString {
		// return OsString::from_vec (self.0.into ());
		return OsString::from (String::from (self.0));
	}
}

impl From<&str> for StringDescriptor {
	fn from (_string : &str) -> Self {
		// return StringDescriptor (OsString::from (_string) .into_vec () .into_boxed_slice ())
		return StringDescriptor (String::from (_string) .into_boxed_str ());
	}
}




serializable! (ProcessDescriptor : Serializable);
serializable! (CommandDescriptor : Serializable);
serializable! (EnvironmentDescriptor : Serializable);

