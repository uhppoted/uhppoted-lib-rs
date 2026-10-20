#[path = "encode.rs"]
pub mod encode;

#[path = "net.rs"]
pub mod net;

#[path = "error.rs"]
pub mod error;

pub fn find_controllers() -> u64 {
    if let Ok(request) = encode::get_controller_request(0) {
        net::dump(&request);
    }

    0
}

#[cfg(test)]
mod generated_tests {
    // use super::*;
    // include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
    include!("generated/generated_tests.rs");
}
