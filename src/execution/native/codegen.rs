use super::{executor::NativeExecutor, x86_64::X86_64CodeGen};
use crate::syntax::Instruction;
use dynasmrt::{Assembler, AssemblyOffset};

pub struct CodeGeneration<B>
where
    B: NativeCodeGenBackend,
{
    codegen: B,
    ops: dynasmrt::Assembler<B::Relocation>,
}

impl CodeGeneration<X86_64CodeGen> {
    pub fn x86_x64() -> CodeGeneration<X86_64CodeGen> {
        CodeGeneration {
            codegen: X86_64CodeGen,
            ops: dynasmrt::x64::Assembler::new().unwrap(),
        }
    }
}

impl<B> CodeGeneration<B>
where
    B: NativeCodeGenBackend,
{
    pub fn generate(mut self, instrs: impl IntoIterator<Item = Instruction>) -> NativeExecutor {
        let code_start = self.codegen.generate_prolouge(&mut self.ops);

        for instr in instrs {
            self.codegen.generate_instruction(&mut self.ops, &instr);
        }

        self.codegen.generate_epilouge(&mut self.ops);

        match self.ops.finalize() {
            Ok(code) => NativeExecutor::new(code, code_start),
            Err(_) => panic!("Failed to finalize code"),
        }
    }
}

pub trait NativeCodeGenBackend {
    type Relocation: dynasmrt::relocations::Relocation;

    fn generate_prolouge(&self, ops: &mut Assembler<Self::Relocation>) -> AssemblyOffset;

    fn generate_epilouge(&self, ops: &mut Assembler<Self::Relocation>);

    fn generate_instruction(&self, ops: &mut Assembler<Self::Relocation>, instr: &Instruction) {
        match instr {
            Instruction::Add { value } => self.generate_increment(ops, value.0),
            Instruction::Move { value } => self.generate_cell_increment(ops, *value),
            Instruction::Loop { nodes } => self.generate_loop(ops, nodes),
            Instruction::Write => self.generate_write(ops),
            Instruction::Read => self.generate_read(ops),
            Instruction::Set { value } => self.generate_set(ops, *value),
        }
    }

    fn generate_increment(&self, ops: &mut Assembler<Self::Relocation>, value: i8);

    fn generate_cell_increment(&self, ops: &mut Assembler<Self::Relocation>, value: i32);

    fn generate_loop(&self, ops: &mut Assembler<Self::Relocation>, nodes: &[Instruction]);

    fn generate_write(&self, ops: &mut Assembler<Self::Relocation>);

    fn generate_read(&self, ops: &mut Assembler<Self::Relocation>);

    fn generate_set(&self, ops: &mut Assembler<Self::Relocation>, value: u8);
}
