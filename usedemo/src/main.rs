use rand::Rng;
use std::collections::*; // 使用*可以把路径中所有的公共条目都引入到作用域；（谨慎使用）
use std::fmt::Result;
use std::io::{self, Read, Result as IoResult, Write}; // 给条目取别名

fn f1() -> Result {
    Result::Ok(())
}

fn f2() -> IoResult<()> {
    IoResult::Ok(())
}

fn main() {}
