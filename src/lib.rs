#![deny(rust_2018_idioms)]
#![allow(clippy::result_large_err)]

pub use context::Context;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChangeLog {
    pub sections: Vec<changelog::Section>,
}

pub mod changelog;
pub mod command;
pub(crate) mod commit;

mod api;
pub(crate) mod bat;
mod context;
mod crates_index;
pub(crate) mod git;
pub(crate) mod traverse;
mod utils;
pub mod version;
