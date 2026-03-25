trait Vulnerability {
    fn check(&self) -> CheckResult;
    fn setdescription(&self);
    fn setpoints(&self);
    fn setseverity(&self);
    fn setcompleted(&self);
}

struct CheckResult {
    completed: bool,
    description: String,
    severity: String,
    points: u32
}

impl CheckUsers for Vulnerability {
    fn check(&self) -> CheckResult {
        CheckResult {
            completed: false,
            description: "".to_string(),
            severity: "High".to_string(),
            points: 10
        }
    }

fn main() {
    println!("Hello, world!");
}
