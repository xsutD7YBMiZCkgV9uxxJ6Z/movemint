// This is a placehoder file

extern crate abci;

use config;
use vm_runtime;

// Move VM

#[allow(dead_code)]
struct VM {
    runtime: vm_runtime::MoveVM,
}

impl Default for VM {
    fn default() -> VM {
        VM {
            runtime: vm_runtime::MoveVM::new(&config::config::VMConfig::default()),
        }
    }
}

// ABCI

struct ABCIapp;
impl abci::Application for ABCIapp {}

// TODO: Abscissa
// Examples: see `cargo-move` and `cargo-movemint` in the root directory

#[allow(unused_variables)]
fn main() {
    let movevm = VM::default(); // MoveVM with default config
                                // TODO: verifer (`VMVerifier`) and executor (`VMExecutor`)

    abci::run_local(ABCIapp);
}

