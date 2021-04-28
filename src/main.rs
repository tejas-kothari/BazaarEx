use fluence::fce;
use fluence::module_manifest;

mod db;

module_manifest!();

pub fn main() {}

#[fce]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}