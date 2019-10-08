//! Main entry point for CargoMove

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use cargo_move::application::APPLICATION;

/// Boot CargoMove
fn main() {
    abscissa_core::boot(&APPLICATION);
}
