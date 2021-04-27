use fluence::fce;                               // 1
use fluence::module_manifest;                   // 2

module_manifest!();                             // 3

pub fn main() {}                                // 4

#[fce]                                          // 5
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}