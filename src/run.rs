use crate::conf::{CmdOptConf, EnvConf};
use crate::util::err::BrokenPipeError;
use crate::util::OptColorWhen;
use regex::Regex;
use runnel::RunnelIoe;
use std::io::{BufRead, Write};

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf, env: &EnvConf) -> anyhow::Result<()> {
    let mut regs: Vec<Regex> = Vec::new();
    for pat in &conf.opt_exp {
        let re = Regex::new(&pat)?;
        regs.push(re);
    }
    //
    let r = do_match_proc(sioe, conf, env, &regs);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn do_match_proc(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    env: &EnvConf,
    regs: &[Regex],
) -> anyhow::Result<()> {
    use naive_opt::Search;
    let color_start_s = env.color_seq_start.as_str();
    let color_end_s = env.color_seq_end.as_str();
    let mut prevs: Vec<String> = Vec::new();
    let mut nexts: Vec<String> = Vec::new();
    let mut prev_found = false;
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
                if conf.flg_inverse {
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
        for needle in conf.opt_str.iter() {
            for (idx, ss) in line_ss.search_indices(needle) {
                b_found = true;
                if conf.flg_inverse {
                    continue 'line_get;
                };
                //
                let st = idx;
                let ed = idx + ss.len();
                for m in line_color_mark.iter_mut().take(ed).skip(st) {
                    *m = true;
                }
            }
        }
        if conf.flg_inverse && !b_found {
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
            continue;
        } else if b_found {
            let s = if let OptColorWhen::Always = conf.opt_color {
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
                out_s.to_string()
            } else {
                line_ss.to_string()
            };
            //
            let mut o = sioe.pout().lock();
            for line in &prevs {
                o.write_fmt(format_args!("{}\n", line))?;
            }
            o.write_fmt(format_args!("{}\n", s))?;
            prevs.clear();
            prev_found = true;
        } else if !conf.opt_around.is_empty() {
            if prev_found {
                nexts.push(line_ss.to_string());
                if nexts.len() >= conf.opt_around.num() {
                    let mut o = sioe.pout().lock();
                    for line in &nexts {
                        o.write_fmt(format_args!("{}\n", line))?;
                    }
                    o.write_fmt(format_args!("\n"))?;
                    nexts.clear();
                    prev_found = false;
                }
            } else {
                prevs.push(line_ss.to_string());
                if prevs.len() > conf.opt_around.num() {
                    let _ = prevs.remove(0);
                }
            };
        };
    }
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
