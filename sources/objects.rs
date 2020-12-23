

use crate::prelude::*;
use crate::lib::*;



#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct ProcessDescriptor {
	pub command : CommandDescriptor,
	pub environment : Option<EnvironmentDescriptor>,
	pub directory : Option<OsString>,
	pub stdio : Option<StdioDescriptor>,
}


#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct CommandDescriptor {
	pub executable : OsString,
	pub argument0 : Option<OsString>,
	pub arguments : Option<Box<[OsString]>>,
}


#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct EnvironmentDescriptor {
	pub inherit : bool,
	pub variables : Option<Box<[VariableDescriptor]>>,
}


#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub enum VariableDescriptor {
	Include (OsString),
	Exclude (OsString),
	Override (OsString, OsString),
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


serializable! (ProcessDescriptor : Serializable);
serializable! (CommandDescriptor : Serializable);
serializable! (EnvironmentDescriptor : Serializable);

