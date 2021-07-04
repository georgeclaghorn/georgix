pub struct TestDescAndFn {
    pub desc: TestDesc,
    pub testfn: StaticTestFn,
}

pub struct TestDesc {
    pub allow_fail: bool,
    pub ignore: bool,
    pub name: StaticTestName,
    pub should_panic: ShouldPanic,
    pub test_type: TestType,
    pub compile_fail: bool,
    pub no_run: bool
}

pub struct StaticTestName(pub &'static str);

impl core::fmt::Display for StaticTestName {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Copy, Clone)]
pub struct StaticTestFn(pub fn());

impl core::ops::FnOnce<()> for StaticTestFn {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.0()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestType {
    UnitTest,
    IntegrationTest,
    DocTest,
    Unknown
}

#[derive(PartialEq)]
pub enum ShouldPanic {
    No,
    Yes
}

pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}
