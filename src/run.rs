use crate::conf::CmdOptConf;
use crate::util::err::BrokenPipeError;
use crate::util::OptColorWhen;
use regex::Regex;
use runnel::RunnelIoe;
use std::io::{BufRead, Write};

/*
use regex::Regex;

use crate::conf::CmdOptConf;
use crate::util::AppError;
use crate::util::OptColorWhen;

use std::io;
use std::io::BufRead;
use std::io::Write;
*/

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let mut regs: Vec<Regex> = Vec::new();
    for pat in &conf.opt_patterns {
        let re = Regex::new(&pat)?;
        regs.push(re);
    }
    //
    let r = do_match_proc(sioe, conf, &regs);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn do_match_proc(sioe: &RunnelIoe, conf: &CmdOptConf, regs: &[Regex]) -> anyhow::Result<()> {
    let color_start_s = conf.opt_color_seq_start.as_str();
    let color_end_s = conf.opt_color_seq_end.as_str();
    //
    'line_get: for line in sioe.pin().lock().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        let line_len: usize = line_ss.len();
        //
        let mut line_color_mark: Vec<bool> = Vec::with_capacity(line_len);
        line_color_mark.resize(line_len, false);
        let mut b_found = false;
        //
        for re in regs {
            for mat in re.find_iter(line_ss) {
                b_found = true;
                if conf.flg_invert_match {
                    continue 'line_get;
                };
                //
                let st = mat.start();
                let ed = mat.end();
                for m in line_color_mark.iter_mut().take(ed).skip(st) {
                    *m = true;
                }
            }
        }
        if b_found {
            if let OptColorWhen::Always = conf.opt_color_when {
                let mut out_s: String = String::new();
                let mut color = false;
                let mut st: usize = 0;
                loop {
                    let next_pos = match line_color_mark.iter().skip(st).position(|c| *c != color) {
                        Some(pos) => st + pos,
                        None => line_len,
                    };
                    if st != next_pos {
                        if color {
                            out_s.push_str(color_start_s);
                        }
                        out_s.push_str(&line_ss[st..next_pos]);
                        if color {
                            out_s.push_str(color_end_s);
                        }
                    }
                    //
                    if next_pos >= line_len {
                        break;
                    }
                    st = next_pos;
                    color = line_color_mark[st];
                }
                #[rustfmt::skip]
                sioe.pout().lock().write_fmt(format_args!("{}\n", out_s))?;
            } else {
                #[rustfmt::skip]
                sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
            }
        } else if conf.flg_invert_match {
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
        };
    }
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
