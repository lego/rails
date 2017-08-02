use super::task_descriptor::TaskDescriptor;

const MAX_PRIORITY: u32 = 32;

struct Node<T> {
  next: Option<&Node<T>>,
  prev: Option<&Node<T>>,
  data: T,
}

struct Priority {
  next: Node<&TaskDescriptor>,
}

pub struct SchedulerState {
  priorities_ready: [bool; 32],
  priorities_ready: [bool; 32],
}
