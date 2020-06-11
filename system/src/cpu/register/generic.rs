use crate::utils::Unsigned;


// It feels extremely unnecessary to me to wrap a simple primitive in a struct,
// if you don't agree, feel free to change it.

// Just replace the type for a struct, and modify all usages in
// super::super::opcode::behaviour::{operations, addressing}

#[allow(type_alias_bounds)] //the bounds are just for reference, the compiler ignores them on types
pub type GenericRegister<T: Unsigned> = T;