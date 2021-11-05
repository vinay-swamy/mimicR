use extendr_api::prelude::*;
use std::collections::HashMap;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

fn parse_snomed_ff(file:&str) -> HashMap<String, String>{
    let mut

    return HashMap::new()
}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod mimicR;
    fn hello_world;
}
