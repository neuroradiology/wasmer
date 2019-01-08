#[macro_use]
mod macros;
mod backend;
mod backing;
mod instance;
mod memory;
mod module;
mod sig_registry;
mod table;
mod recovery;
mod sighandler;
mod mmap;
pub mod types;
pub mod vm;
pub mod vmcalls;

pub use self::backend::{Compiler, FuncResolver};
pub use self::instance::{Import, ImportResolver, Imports, Instance};
pub use self::module::Module;
pub use self::sig_registry::SigRegistry;
pub use self::memory::LinearMemory;

/// Compile a webassembly module using the provided compiler.
pub fn compile(
    wasm: &[u8],
    compiler: &dyn Compiler,
) -> Result<Module, String> {
    compiler.compile(wasm)
}