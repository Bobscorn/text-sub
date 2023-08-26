use std::clone::*;
use std::marker::*;

#[derive(Copy, Clone)]
pub struct SubPart {
    pub symbol: char,
    pub label: &'static str,
    pub description: &'static str 
}
