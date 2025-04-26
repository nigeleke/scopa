#![feature(coverage_attribute)]

use scopa::App;

#[coverage(off)]
fn main() {
    dioxus::launch(App);
}
