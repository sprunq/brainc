pub mod peephole;

use crate::syntax::Instruction;
use itertools::Itertools;

pub trait OptimizationPass {
    fn optimize(
        &self,
        nodes: impl IntoIterator<Item = Instruction>,
    ) -> impl Iterator<Item = Instruction>;
}

pub trait OptimizeExt: Iterator<Item = Instruction> + Sized {
    fn optimize(self, pass: &impl OptimizationPass) -> impl Iterator<Item = Instruction> {
        pass.optimize(self)
    }
}

impl<I> OptimizeExt for I where I: Iterator<Item = Instruction> {}

trait MapLoopsExt: Iterator<Item = Instruction> {
    fn map_loops<O>(self, optimize_with: O) -> MapLoops<Self, O>
    where
        O: OptimizationPass,
        Self: Sized,
    {
        MapLoops {
            iter: self,
            optimizer: optimize_with,
        }
    }
}

struct MapLoops<I, O> {
    iter: I,
    optimizer: O,
}

impl<'a, I, O> Iterator for MapLoops<I, O>
where
    I: Iterator<Item = Instruction>,
    O: OptimizationPass,
{
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|instr| match instr {
            Instruction::Loop { nodes } => Instruction::Loop {
                nodes: self.optimizer.optimize(nodes).collect_vec(),
            },
            other => other,
        })
    }
}

impl<I> MapLoopsExt for I where I: Iterator<Item = Instruction> {}
