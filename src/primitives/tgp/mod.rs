//! Tree based genetic programming primitives

use rand::Rng;

use std::collections::{HashSet, VecDeque};

pub enum Primitive<T>
{
    Constant(T),
    EphemeralConstant(fn(&mut Rng) -> T),
    Operation {
        arity: usize,
        op: fn(Vec<Primitive<T>>) -> Primitive<T>
    }
}

pub struct Tree<T>
{
    value: Primitive<T>,
    children: Vec<Box<Tree<T>>>
}

impl<T> Tree<T>
{
}

/// Compile a tree containing ephemeral constants into an evaluated tree
pub fn compile<T>(a: &Tree<T>) -> Tree<T>
{
    unimplemented!();
}
