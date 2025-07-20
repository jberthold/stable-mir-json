//! This module provides a compiler driver such that:
//!
//! 1.  the rustc compiler context is available
//! 2.  the rustc `stable_mir` APIs are available
//!
//! It exports a single function:
//!
//! ```rust,ignore
//! stable_mir_driver(args: &Vec<String>, callback_fn: fn (TyCtxt) -> () )
//! ```
//!
//! Calling this function is essentially equivalent to the following macro call:
//!
//! ```rust,ignore
//! rustc_internal::run_with_tcx!( args, callback_fn );
//! ```
//!
//! However, we prefer a non-macro version for clarity and build simplicity.

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;

// Check if we can access stable_mir and rustc_smir differently  
// We'll try both old and new patterns
use rustc_driver::{Compilation, run_compiler};
use rustc_interface::interface::{Compiler, Config};
use rustc_middle::ty::TyCtxt;

struct StableMirCallbacks {
    callback_fn: fn(TyCtxt) -> (),
}

impl rustc_driver::Callbacks for StableMirCallbacks {
    fn after_analysis(&mut self, _compiler: &Compiler, tcx: TyCtxt) -> Compilation {
        // Try to access stable_mir functionality
        // We'll need to adapt this based on what's available
        (self.callback_fn)(tcx);
        Compilation::Continue
    }
}

pub fn stable_mir_driver(args_outer: &[String], callback_fn: fn(TyCtxt) -> ()) {
    let mut callbacks = StableMirCallbacks { callback_fn };
    
    // Use the new run_compiler API
    run_compiler(args_outer, &mut callbacks);
}
