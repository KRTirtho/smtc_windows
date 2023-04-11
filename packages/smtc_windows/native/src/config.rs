#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SMTCConfig{
  pub play_enabled: bool,
  pub pause_enabled: bool,
  pub stop_enabled: bool,
  pub next_enabled: bool,
  pub prev_enabled: bool,
  pub fast_forward_enabled: bool,
  pub rewind_enabled: bool,
}

impl Default for SMTCConfig {
  fn default() -> Self {
    Self {
      play_enabled: true,
      pause_enabled: true,
      stop_enabled: true,
      next_enabled: true,
      prev_enabled: true,
      fast_forward_enabled: false,
      rewind_enabled: false,
    }
  }
}