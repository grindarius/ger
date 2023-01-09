/// swagger api documentation registration module.
pub mod apidoc;
/// swagger apikey middleware module for detecting swagger api key in headers for testing in
/// swagger ui.
pub mod apikey_middleware;
/// module for logging swagger api key.
pub mod log_apikey;
/// module for requiring swagger api key.
pub mod require_apikey;
/// module for adding swagger api key checks to the api.
pub mod security_addon;
