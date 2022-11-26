// This conditional compilation must be in a sub-module because code within a `cfg` block must parse
// and `impl const From...` will not parse until the `const_trait_impl` stabilizes.  Rust skips
// parsing of conditionally compiled out submodules.

pub mod msg;
