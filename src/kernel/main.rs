use mach::{MachineState, Machine};
use mach::io::{bwputstr, bwgetc, bwputc};

#[no_mangle]
pub extern "C" fn kernel_main() -> i32 {
  let mut machine = MachineState::new();
  machine.initialize();


  bwputstr("Hello world!\n");
  loop {
    let c: char = bwgetc();
    if c == 'q' {
      break;
    }
    bwputc(c);
  }
  bwputstr("Goodbye!\n");


  // create initial task

  loop {
    // get next task to schedule
    break;
    // switch to task
    // handle task request
  }

  machine.cleanup();
  return 0;
}
