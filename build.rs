#[cfg(feature = "skeptic")]
extern crate skeptic;

#[cfg(feature = "skeptic")]
fn main() {
    skeptic::generate_doc_tests(&["README.md"]);
}

#[cfg(not(feature = "skeptic"))]
fn main() {}
