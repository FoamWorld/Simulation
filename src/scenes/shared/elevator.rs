use std::vec::Vec;

enum ElevatorStatus {
    Pausing,
    Ascending(i32),
    Descending(i32),
}

struct Elevator {
    current: i32,
    status: ElevatorStatus,
    open: bool,
    contents: Vec<i32>,
}

impl Elevator {
    fn register_inner_prompt(&mut self, target: i32) {
        if target > self.current {
            self.status = ElevatorStatus::Ascending(target);
        } else if target < self.current {
            self.status = ElevatorStatus::Descending(target);
        }
    }
    fn register_prompt(level: i32) {}
    fn cancel_prompt(level: i32) {}
}
