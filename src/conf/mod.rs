pub use self::parse::parse_cmdopts;

mod parse;

use crate::util::OptColorWhen;
use std::default::Default;
use std::env;

#[derive(Debug, Default)]
pub struct CmdOptConf {
    pub opt_program: String,
    //
    pub flg_help: bool,
    pub flg_version: bool,
    //
    pub flg_invert_match: bool,
    //
    pub opt_patterns: Vec<String>,
    pub opt_color_when: OptColorWhen,
    pub opt_color_seq_start: String,
    pub opt_color_seq_end: String,
    //
    pub arg_params: Vec<String>,
}

impl flood_tide::HelpVersion for CmdOptConf {
    fn is_help(&self) -> bool {
        self.flg_help
    }
    fn is_version(&self) -> bool {
        self.flg_version
    }
}

static COLOR_START: &str = "\u{1B}[01;31m";
static COLOR_END: &str = "\u{1B}[0m";

impl CmdOptConf {
    pub fn create(program: &str) -> Self {
        let a_color_start = match env::var("RUST_GREP_COLOR_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_START),
        };
        let a_color_end = match env::var("RUST_GREP_COLOR_ED") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_END),
        };
        //
        Self {
            opt_program: program.to_string(),
            opt_color_seq_start: a_color_start,
            opt_color_seq_end: a_color_end,
            //
            ..Default::default()
        }
    }
}