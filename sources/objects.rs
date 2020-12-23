

use crate::prelude::*;
use crate::lib::*;




#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct ProcessDescriptor {
	
	pub command : CommandDescriptor,
	
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub environment : Option<EnvironmentDescriptor>,
	
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub directory : Option<StringDescriptor>,
	
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub stdio : Option<StdioDescriptor>,
}


#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct CommandDescriptor {
	
	pub executable : StringDescriptor,
	
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub argument0 : Option<StringDescriptor>,
	
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub arguments : Option<Box<[StringDescriptor]>>,
}


#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct EnvironmentDescriptor {
	
	pub inherit : bool,
	
	#[ serde (skip_serializing_if = "Option::is_none") ]
	pub variables : Option<Box<[VariableDescriptor]>>,
}


#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
#[ serde (tag = "action") ]
pub enum VariableDescriptor {
	
	#[ serde (rename = "include") ]
	Include { key : StringDescriptor },
	
	#[ serde (rename = "exclude") ]
	Exclude { key : StringDescriptor },
	
	#[ serde (rename = "override") ]
	Override { key : StringDescriptor, value : StringDescriptor },
}

#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct StdioDescriptor {
	pub stdin : IoDescriptor,
	pub stdout : IoDescriptor,
	pub stderr : IoDescriptor,
}


#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub enum IoDescriptor {
	Inherit,
	DevNull,
}




#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct StringDescriptor (
	
	// FIXME:  Add support for non UTF-8 strings!
	// #[ serde (with = "serde_bytes") ]
	// Box<[u8]>,
	
	Box<str>,
);

impl AsRef<OsStr> for StringDescriptor {
	fn as_ref (&self) -> &OsStr {
		return OsStr::new (self.0.as_ref ());
	}
}

impl Into<OsString> for StringDescriptor {
	fn into (self) -> OsString {
		return OsString::from (String::from (self.0));
	}
}

impl From<OsString> for StringDescriptor {
	fn from (_string : OsString) -> Self {
		return StringDescriptor (String::from (_string.to_string_lossy ()) .into_boxed_str ());
	}
}

impl From<&str> for StringDescriptor {
	fn from (_string : &str) -> Self {
		return StringDescriptor (String::from (_string) .into_boxed_str ());
	}
}

impl From<Vec<u8>> for StringDescriptor {
	fn from (_string : Vec<u8>) -> Self {
		return OsString::from_vec (_string) .into ();
	}
}




serializable! (ProcessDescriptor : Serializable);
serializable! (CommandDescriptor : Serializable);
serializable! (EnvironmentDescriptor : Serializable);

