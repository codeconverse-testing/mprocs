use super::{
  msg::{ProcCmd, ProcEvent},
  CopyMode, Proc,
};

pub struct ProcHandle {
  id: usize,
  name: String,
  is_up: bool,

  pub to_restart: bool,
  changed: bool,

  proc: Proc,
  exit_code: Option<i32>, // Add an attribute to store the exit code
}

impl ProcHandle {
  pub fn from_proc(name: String, proc: Proc) -> Self {
    Self {
      id: proc.id,
      name,
      is_up: false,
      to_restart: false,
      changed: false,
      proc,
      exit_code: None, // Initialize the exit code as None
    }
  }

  // Implement a method to set the exit code
  pub fn set_exit_code(&mut self, code: i32) {
    self.exit_code = Some(code);
  }

  // Implement a method to retrieve the exit code
  pub fn exit_code(&self) -> Option<i32> {
    self.exit_code
  }

  pub fn send(&mut self, cmd: ProcCmd) {
    self.proc.handle_cmd(cmd)
  }

  pub fn rename(&mut self, name: &str) {
    self.name.replace_range(.., &name);
  }

  pub fn id(&self) -> usize {
    self.id
  }

  pub fn lock_view(&self) -> ProcViewFrame {
    match &self.proc.inst {
      super::ProcState::None => ProcViewFrame::Empty,
      super::ProcState::Some(inst) => inst
        .vt
        .read()
        .map_or(ProcViewFrame::Empty, |vt| ProcViewFrame::Vt(vt)),
      super::ProcState::Error(err) => ProcViewFrame::Err(&err),
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn is_up(&self) -> bool {
    self.is_up
  }

  pub fn changed(&self) -> bool {
    self.changed
  }

  pub fn copy_mode(&self) -> &CopyMode {
    &self.proc.copy_mode
  }

  pub fn focus(&mut self) {
    self.changed = false;
  }

  pub fn handle_event(&mut self, event: ProcEvent, selected: bool) {
    match event {
      ProcEvent::Render => {
        if !selected {
          self.changed = true;
        }
      }
      ProcEvent::Stopped => { // Modified to assume no exit code passed
        self.is_up = false;
        // self.set_exit_code(exit_code); // Set the exit code when the process stops
        if self.to_restart {
          self.to_restart = false;
          self.send(ProcCmd::Start);
        }
      }
      ProcEvent::Started => {
        self.is_up = true;
      }
      // Other event handling...
    }
  }
}

pub enum ProcViewFrame<'a> {
  Empty,
  Vt(std::sync::RwLockReadGuard<'a, vt100::Parser>),
  Err(&'a str),
}
