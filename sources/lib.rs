
#![ allow (unused_imports) ]
#![ allow (unused_macros) ]
#![ allow (dead_code) ]




#[ macro_use ]
mod macros;


pub(crate) mod main;
pub(crate) mod prelude;
pub(crate) mod objects;
pub(crate) mod settings;
pub(crate) mod tools;




pub use self::main::main;




pub mod lib {
	
	#![ allow (unused_imports) ]
	
	pub use crate::main::*;
	pub use crate::objects::*;
	pub use crate::settings::*;
	pub use crate::tools::*;
}

