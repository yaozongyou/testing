pub use plugins::testing;

pub struct TestDescAndFn {
    name: &'static str,
    comment: &'static str,
    source_file: &'static str,
    start_line: usize,
    test_fn: fn(),
}

impl TestDescAndFn {
    pub const fn new(
        name: &'static str,
        comment: &'static str,
        source_file: &'static str,
        start_line: usize,
        test_fn: fn(),
    ) -> Self {
        TestDescAndFn {
            name,
            comment,
            source_file,
            start_line,
            test_fn,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn comment(&self) -> &'static str {
        self.comment
    }

    pub fn source_file(&self) -> &'static str {
        self.source_file
    }

    pub fn start_line(&self) -> usize {
        self.start_line
    }

    pub fn run(&self) {
        (self.test_fn)()
    }
}

inventory::collect!(TestDescAndFn);
