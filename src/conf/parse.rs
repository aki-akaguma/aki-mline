//
//use flood_tide::parse_simple_gnu_style;
use flood_tide::HelpVersion;
use flood_tide::{Arg, Lex, NameVal, Opt, OptNum};
use flood_tide::{OptParseError, OptParseErrors};

use crate::conf::CmdOptConf;
use crate::util::OptColorWhen;

use std::str::FromStr;

//----------------------------------------------------------------------
include!("cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
match line, regex text filter, like grep.
"#;
//const ARGUMENTS_TEXT: &str = r#""#;
const ENV_TEXT: &str = r#"Env:
  RUST_GREP_COLOR_ST   color start sequence
  RUST_GREP_COLOR_ED   color end sequence
"#;
//const EXAMPLES_TEXT: &str = r#""#;
//}}} TEXT

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}", program, "[options]")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message(env!("CARGO_PKG_NAME"));
    //[ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, ARGUMENTS_TEXT, ENV_TEXT, EXAMPLES_TEXT].join("\n")
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, ENV_TEXT].join("\n")
}

//----------------------------------------------------------------------
fn value_to_color_when(nv: &NameVal<'_>) -> Result<OptColorWhen, OptParseError> {
    match nv.val {
        Some(s) => match FromStr::from_str(s) {
            Ok(color) => Ok(color),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Ok(OptColorWhen::Auto),
    }
}

//----------------------------------------------------------------------
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    match CmdOp::from(nv.opt.num) {
        CmdOp::Color => {
            //conf.opt_color = value_to_string(nv)?;
            conf.opt_color_when = value_to_color_when(nv)?;
        }
        CmdOp::Exp => {
            let pat = value_to_string(nv)?;
            conf.opt_patterns.push(pat);
        }
        CmdOp::InvertMatch => {
            conf.flg_invert_match = true;
        }
        CmdOp::Help => {
            conf.flg_help = true;
        }
        CmdOp::Version => {
            conf.flg_version = true;
        }
    }
    //
    Ok(())
}

pub fn parse_my_style<'a, T, F>(
    conf: &mut T,
    opt_ary: &'a [Opt],
    sho_idx_ary: &'a [(u8, usize)],
    args: &'a [&'a str],
    parse_match: F,
) -> (Option<Vec<String>>, Result<(), OptParseErrors>)
where
    F: Fn(&mut T, &NameVal<'_>) -> Result<(), OptParseError>,
    T: HelpVersion,
{
    let lex = Lex::create_with(opt_ary, sho_idx_ary);
    let tokens = match lex.tokens_from(&args) {
        Ok(t) => t,
        Err(errs) => {
            return (None, Err(errs));
        }
    };
    //
    let mut errs = OptParseErrors::new();
    //
    for nv in tokens.namevals.iter() {
        match parse_match(conf, &nv) {
            Ok(_) => {}
            Err(err) => {
                errs.push(err);
            }
        }
        if conf.is_help() || conf.is_version() {
            break;
        }
    }
    //
    let mut v: Vec<String> = Vec::new();
    v.extend(tokens.free.iter().map(|&s| s.to_string()));
    //
    (Some(v), Err(errs))
}

pub fn parse_cmdopts(a_prog_name: &str, args: &[&str]) -> Result<CmdOptConf, OptParseErrors> {
    //
    let mut conf = CmdOptConf::create(a_prog_name);
    let (opt_free, r_errs) =
        parse_my_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, args, parse_match);
    //
    if conf.is_help() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::help_message(&help_message(
            &conf.prog_name,
        )));
        return Err(errs);
    }
    if conf.is_version() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::version_message(&version_message(
            &conf.prog_name,
        )));
        return Err(errs);
    }
    //
    {
        let mut errs = if let Err(errs) = r_errs {
            errs
        } else {
            OptParseErrors::new()
        };
        //
        if conf.opt_patterns.is_empty() {
            errs.push(OptParseError::missing_option("e"));
        }
        if conf.opt_color_when == OptColorWhen::Auto {
            if atty::is(atty::Stream::Stdout) {
                conf.opt_color_when = OptColorWhen::Always;
            } else {
                conf.opt_color_when = OptColorWhen::Never;
            }
        }
        //
        if let Some(free) = opt_free {
            if !free.is_empty() {
                errs.push(OptParseError::unexpected_argument(&free[0]));
            }
        };
        if !errs.is_empty() {
            return Err(errs);
        }
    }
    //
    Ok(conf)
}
