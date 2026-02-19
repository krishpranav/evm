pub struct MiniVm {
  pc: usize,
  stack: Vec<u64>,
  code: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum ExecResult {
  Stopped,
  EndOfCode,
  Error(VmError),
}

#[derive(Debug, PartialEq)]
pub enum VmError {
  StackUnderflow,
  UnknownOpcode(u8),
  MissingPushData,
}

impl MiniVm {
  pub fn new(code: Vec<u8>) -> Self {
    Self {
      pc: 0,
      stack: Vec::new(),
      code,
    }
  }

  pub fn run(&mut self) -> ExecResult {
    loop {
      if self.pc >= self.code.len() {
        return ExecResult::EndOfCode;
      }

      let opcode = self.code[self.pc];

      self.pc += 1;

      match opcode {
        0x00 => return ExecResult::Stopped,

        0x01 => {
          let a = match self.pop() {
            Ok(v) => v,
            Err(e) => return ExecResult::Error(e),
          };
          let b = match self.pop() {
            Ok(v) => v,
            Err(e) => return ExecResult::Error(e),
          };

          self.stack.push(a.wrapping_add(b));
        }

        0x60 => {
          if self.pc >= self.code.len() {
            return ExecResult::Error(VmError::MissingPushData);
          }

          let value = self.code[self.pc] as u64;

          self.pc += 1;
          self.stack.push(value);
        }

        other => return ExecResult::Error(VmError::UnknownOpcode(other)),
      }
    }
  }

  fn pop(&mut self) -> Result<u64, VmError> {
    self.stack.pop().ok_or(VmError::StackUnderflow)
  }

  pub fn peek(&self) -> Option<u64> {
    self.stack.last().copied()
  }

  pub fn stack(&self) -> &[u64] {
    &self.stack
  }
}