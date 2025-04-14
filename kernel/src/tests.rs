pub struct TestCase {
  pub name: &'static str,
  pub function: fn() -> Result<(), &'static str>,
}

pub fn run_tests(tests: &[TestCase]) {

}