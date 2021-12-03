// current specification version
static VERSION: u8 = 3;

#[derive(Debug)]
pub struct SourceMap {
  pub version: u8,
  pub mappings: String,
}

impl SourceMap {
  pub fn new(mappings: &str) -> Self {
    Self {
      version: VERSION,
      mappings: String::from(mappings),
    }
  }
}

// impl Default for SourceMap {
//   fn default() -> Self {
//     Self { version: VERSION }
//   }
// }
