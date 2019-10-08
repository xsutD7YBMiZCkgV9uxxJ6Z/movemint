//! Main entry point for CargoMovemint

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use cargo_movemint::application::APPLICATION;

/// Boot CargoMovemint
fn main() {
    abscissa_core::boot(&APPLICATION);
}
