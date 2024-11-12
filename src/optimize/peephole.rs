use super::{MapLoopsExt, OptimizationPass};
use crate::syntax::Instruction;
use crate::syntax::Instruction::*;
use itertools::Itertools;

pub struct CombineIncrements;

impl OptimizationPass for CombineIncrements {
    fn optimize(
        &self,
        nodes: impl IntoIterator<Item = Instruction>,
    ) -> impl Iterator<Item = Instruction> {
        nodes
            .into_iter()
            .map_loops(Self)
            .coalesce(|prev, current| match (prev, current) {
                (Add { value: a }, Add { value: b }) => Ok(Add { value: a + b }),
                (Move { value: a }, Move { value: b }) => Ok(Move { value: a + b }),
                (a, b) => Err((a, b)),
            })
    }
}

pub struct ReplaceSet;

impl OptimizationPass for ReplaceSet {
    fn optimize(
        &self,
        nodes: impl IntoIterator<Item = Instruction>,
    ) -> impl Iterator<Item = Instruction> {
        nodes.into_iter().map_loops(Self).map(|instr| {
            if let Loop { ref nodes } = instr {
                if nodes.len() == 1 {
                    let inner = &nodes[0];
                    if let Add { value } = inner {
                        let value = value.0;
                        if value == -1 || value == 1 {
                            return Set { value: 0 };
                        }
                    }
                }
            }
            instr
        })
    }
}

/// Combines consecutive `Set` instructions into a single `Set` instruction.
/// Combines `Set` and `Add` instructions into a single `Set` instruction.
/// Combines `Add` and `Set` instructions into a single `Set` instruction with the Add value being discarded.
pub struct CombineSets;

impl OptimizationPass for CombineSets {
    fn optimize(
        &self,
        nodes: impl IntoIterator<Item = Instruction>,
    ) -> impl Iterator<Item = Instruction> {
        nodes
            .into_iter()
            .map_loops(Self)
            .coalesce(|prev, current| match (prev, current) {
                (Set { value: a }, Set { value: b }) => Ok(Set { value: a + b }),
                (Set { value: a }, Add { value: b }) => Ok(Set {
                    value: a.wrapping_add_signed(b.0),
                }),
                (Add { value: _ }, Set { value: b }) => Ok(Set { value: b }),
                (a, b) => Err((a, b)),
            })
    }
}
