pub struct TaskDescriptor {
  tid: u32,
  parent_tid: u32,
  has_started: bool,
  priority: u32,
  entrypoint: Fn(),
}
