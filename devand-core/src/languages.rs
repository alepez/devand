use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Copy,
    Clone,
    EnumIter,
    Display,
    EnumString,
)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Ada,
    Bash,
    C,
    Caml,
    #[strum(serialize = "C++")]
    CPlusPlus,
    #[strum(serialize = "C#")]
    CSharp,
    Clojure,
    Dart,
    Elixir,
    Erlang,
    #[strum(serialize = "F#")]
    FSharp,
    Go,
    Groovy,
    Haskell,
    Java,
    JavaScript,
    Kotlin,
    Lisp,
    ObjectiveC,
    PHP,
    Python,
    R,
    Ruby,
    Rust,
    Scala,
    Swift,
    TypeScript,
    VBA,
}
