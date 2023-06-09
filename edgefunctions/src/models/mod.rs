pub mod bad_request_response;
pub use self::bad_request_response::BadRequestResponse;
pub mod create_edge_function_request;
pub use self::create_edge_function_request::CreateEdgeFunctionRequest;
pub mod edge_function_response;
pub use self::edge_function_response::EdgeFunctionResponse;
pub mod error_response;
pub use self::error_response::ErrorResponse;
pub mod links;
pub use self::links::Links;
pub mod list_edge_function_response;
pub use self::list_edge_function_response::ListEdgeFunctionResponse;
pub mod patch_edge_function_request;
pub use self::patch_edge_function_request::PatchEdgeFunctionRequest;
pub mod put_edge_function_request;
pub use self::put_edge_function_request::PutEdgeFunctionRequest;
pub mod results;
pub use self::results::Results;
