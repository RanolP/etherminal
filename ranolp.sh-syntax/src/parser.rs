use pest_derive::*;

#[derive(Parser)]
#[grammar = "../assets/grammar.pest"] // relative to project `src`
pub struct MyParser;
