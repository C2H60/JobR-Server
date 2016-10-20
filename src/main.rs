extern crate jobr_lib;
use jobr_lib::jobapi;
fn main() {
    // Start new server
    let mut live_api = jobapi::JobApi::new("0.0.0.0:4000".to_string(), false);
    live_api.start_server();
}
