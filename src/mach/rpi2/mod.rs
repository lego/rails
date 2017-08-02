use mach::{Machine, MachineState};

pub mod bwio;
mod io;
mod uart;

pub struct State {}

impl State {
  pub fn new() -> State {
    State {}
  }
}

impl Machine for MachineState {
  fn initialize(&mut self) -> bool {
    io::uart_init();

    self.initialized = true;
    self.initialized
  }

  fn cleanup(&mut self) {}
}
