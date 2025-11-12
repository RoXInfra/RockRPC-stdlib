use rockrpc_method::{CustomMethod, JsonValue, RpcError};

pub struct EchoMethod;
impl CustomMethod for EchoMethod {
	type Params = JsonValue;
	type SuccessData = JsonValue;
	type ErrorData = ();

	async fn run(params: Self::Params) -> Result<Self::SuccessData, RpcError<Self::ErrorData>> {
		Ok(params)
	}
}
rockrpc_method::rockrpc_custom_method!(EchoMethod);
