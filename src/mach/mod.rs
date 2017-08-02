// #[cfg(plat_rpi2)]
mod rpi2;
// #[cfg(plat_rpi2)]
mod state;

pub mod io;
pub mod mmio;

pub trait Machine {
  fn initialize(&mut self) -> bool;
  fn cleanup(&mut self);
}

pub struct MachineState {
  initialized: bool,
  state: state::State,
}

impl MachineState {
  pub fn new() -> MachineState {
    MachineState {
      initialized: false,
      state: state::State::new(),
    }
  }
}

// pub fn create() -> Box<MachineState> {
//     box MachineState::new()
// }
