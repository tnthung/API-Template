#![allow(unused)]


mod r#macro;

use once_cell::sync::Lazy;
use self::r#macro::*;


Var!(RUN_MODE, "RUN_MODE", true, "NONE");
