pub use self::parse::parse_cmdopts;

mod parse;

use crate::util::OptColorWhen;
use std::default::Default;
use std::env;

#[derive(Debug, Default)]
pub struct CmdOptConf {
    pub prog_name: String,
    //
    pub opt_color: OptColorWhen,
    pub opt_exp: Vec<String>,
    pub opt_str: Vec<String>,
    pub flg_inverse: bool,
    pub flg_help: bool,
    pub flg_version: bool,
    //
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
    pub fn create(a_prog_name: &str) -> Self {
        let a_color_start = match env::var("AKI_MLINE_COLOR_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_START),
        };
        let a_color_end = match env::var("AKI_MLINE_COLOR_ED") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_END),
        };
        //
        Self {
            prog_name: a_prog_name.to_string(),
            opt_color_seq_start: a_color_start,
            opt_color_seq_end: a_color_end,
            //
            ..Default::default()
        }
    }
}
