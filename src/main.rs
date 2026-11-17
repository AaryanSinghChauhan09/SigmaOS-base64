//! SigmaOS-base64 — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_base64_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-base64: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-base64: ok");
}
