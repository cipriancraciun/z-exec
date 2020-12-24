

use crate::prelude::*;
use crate::lib::*;




pub trait RpcRequest : Serializable {
	
	type Response : RpcResponse;
	
	fn wrap (self) -> RpcRequestWrapper;
}

pub trait RpcResponse : Serializable {}




#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub enum RpcRequestWrapper {
	Execute (RpcExecuteRequest),
}

serializable! (RpcRequestWrapper : Serializable);


#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub enum RpcOutcome <Response> {
	Ok (Response),
	Err (String),
}

serializable! (RpcOutcome<Response> : Serializable);




#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct RpcExecuteRequest {
	pub descriptor : ProcessDescriptor
}

impl RpcRequest for RpcExecuteRequest {
	
	type Response = RpcExecuteResponse;
	
	fn wrap (self) -> RpcRequestWrapper {
		RpcRequestWrapper::Execute (self)
	}
}

serializable! (RpcExecuteRequest : Serializable);


#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub struct RpcExecuteResponse {
	pub pid : libc::pid_t,
}

impl RpcResponse for RpcExecuteResponse {}

serializable! (RpcExecuteResponse : Serializable);

