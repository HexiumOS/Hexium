use crate::{info};
pub struct TestCase {
  pub name: &'static str,
  pub function: fn() -> Result<(), &'static str>,
}

pub fn run_tests(tests: &[TestCase]) {
  info!("Running {} tests...", tests.len());

  let mut passed = 0;

  for test in tests {
    info!("Test {}", test.name);

    match (test.function)() {
      Ok(()) => {
        info!("Ok");
        passed += 1;
      },
      Err(msg) => info!("FAILED: {}", msg),
    }
  }

  info!("Passed tests {}, Failed tests {}", passed, tests.len() - passed);
}