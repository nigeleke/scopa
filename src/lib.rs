#![feature(coverage_attribute)]

mod application;
mod domain;
mod i18n;
mod storage;
mod ui;

pub use ui::App;

#[cfg(test)]
mod test_support;
