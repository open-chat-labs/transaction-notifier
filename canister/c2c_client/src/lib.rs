use transaction_notifier::*;

macro_rules! generate_c2c_call {
    ($method_name:ident) => {
        pub async fn $method_name(canister_id: candid::Principal, args: &$method_name::Args) -> ic_cdk::api::call::CallResult<$method_name::Response> {
            let method_name = stringify!($method_name);
            let result: ic_cdk::api::call::CallResult<($method_name::Response,)> = ic_cdk::call(canister_id, method_name, (args,)).await;

            if let Err(error) = &result {
                tracing::error!(method_name, error_code = ?error.0, error_message = error.1.as_str(), "Error calling c2c");
            }

            result.map(|r| r.0)
        }
    };
}

// Queries
generate_c2c_call!(supported_tokens);

// Updates
generate_c2c_call!(add_token);
generate_c2c_call!(subscribe);
generate_c2c_call!(update_token_config);
