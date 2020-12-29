

use crate::prelude::*;
use crate::lib::*;




pub trait RpcRequest : Serializable + fmt::Debug + Send + 'static {
	
	type Response : RpcResponse;
	
	fn wrap (self) -> RpcRequestWrapper;
}

pub trait RpcResponse : Serializable + fmt::Debug + Send + 'static {}




#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub enum RpcRequestWrapper {
	Execute (RpcExecuteRequest),
}

serializable! (RpcRequestWrapper : Serializable);


#[ derive (Debug) ]
#[ derive (serde_derive::Serialize, serde_derive::Deserialize) ]
pub enum RpcOutcome<Response> {
	Ok (Response),
	Err (String),
}

serializable! (RpcOutcome<Response : RpcResponse> : Serializable);


impl <Response : RpcResponseDyn> RpcOutcome<Response> {
	
	pub fn into_boxed (self) -> RpcOutcomeBox {
		match self {
			RpcOutcome::Ok (_response) =>
				RpcOutcomeBox::Ok (Box::new (_response)),
			RpcOutcome::Err (_error) =>
				RpcOutcomeBox::Err (_error),
		}
	}
}




#[ derive (Debug) ]
#[ derive (serde_derive::Serialize) ]
pub enum RpcOutcomeBox {
	Ok (Box<dyn RpcResponseDyn>),
	Err (String),
}

serializable! (RpcOutcomeBox : SerializableOnly);

pub trait RpcResponseDyn : SerializableErased + fmt::Debug + Send + 'static {}

serde_erased::serialize_trait_object! (RpcResponseDyn);

impl <Response : RpcResponse + fmt::Debug + Send + 'static> RpcResponseDyn for Response {}




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
	pub identifier : Identifier,
}

impl RpcResponse for RpcExecuteResponse {}

serializable! (RpcExecuteResponse : Serializable);

