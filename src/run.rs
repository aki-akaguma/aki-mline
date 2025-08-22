use crate::conf::{CmdOptConf, EnvConf};
use crate::util::err::BrokenPipeError;
use crate::util::OptColorWhen;
use regex::Regex;
use runnel::RunnelIoe;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf, env: &EnvConf) -> anyhow::Result<()> {
    let mut regs: Vec<Regex> = Vec::new();
    for pat in &conf.opt_exp {
        let re = Regex::new(pat)?;
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
    let color_start_s = env.color_seq_start.as_str();
    let color_end_s = env.color_seq_end.as_str();
    let mut prevs: Vec<String> = Vec::new();
    let mut nexts: Vec<String> = Vec::new();
    let mut prev_found = false;
    //
    'line_get: for line in sioe.pg_in().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        let (b_continue, b_found, line_color_mark) =
            make_line_color_mark(regs, &conf.opt_str, conf.flg_inverse, line_ss)?;
        if b_continue {
            continue 'line_get;
        }
        if conf.flg_inverse && !b_found {
            sioe.pg_out().write_line(line_s)?;
            continue;
        } else if b_found {
            if !prevs.is_empty() {
                prevs.reverse();
                while let Some(line) = prevs.pop() {
                    sioe.pg_out().write_line(line)?;
                }
            }
            prev_found = true;
            //
            let out_s = if let OptColorWhen::Always = conf.opt_color {
                make_out_s(color_start_s, color_end_s, line_ss, &line_color_mark)?
            } else {
                line_s
            };
            sioe.pg_out().write_line(out_s)?;
        } else if !conf.opt_around.is_empty() {
            if prev_found {
                nexts.push(line_ss.to_string());
                if nexts.len() >= conf.opt_around.num() {
                    nexts.reverse();
                    while let Some(line) = nexts.pop() {
                        sioe.pg_out().write_line(line)?;
                    }
                    sioe.pg_out().write_line("".to_string())?;
                    prev_found = false;
                }
            } else {
                prevs.push(line_s);
                if prevs.len() > conf.opt_around.num() {
                    let _ = prevs.remove(0);
                }
            };
        };
    }
    //
    sioe.pg_out().flush_line()?;
    //
    Ok(())
}

fn make_line_color_mark(
    regs: &[Regex],
    opt_str: &[String],
    flg_inverse: bool,
    line_ss: &str,
) -> anyhow::Result<(bool, bool, Vec<bool>)> {
    use naive_opt::Search;
    let line_len = line_ss.len();
    //
    let mut line_color_mark: Vec<bool> = Vec::with_capacity(line_len);
    line_color_mark.resize(line_len, false);
    let mut b_found = false;
    //
    for re in regs {
        for mat in re.find_iter(line_ss) {
            b_found = true;
            if flg_inverse {
                return Ok((true, b_found, line_color_mark));
            };
            //
            let st = mat.start();
            let ed = mat.end();
            for m in line_color_mark.iter_mut().take(ed).skip(st) {
                *m = true;
            }
        }
    }
    for needle in opt_str.iter() {
        for (idx, ss) in line_ss.search_indices(needle) {
            b_found = true;
            if flg_inverse {
                return Ok((true, b_found, line_color_mark));
            };
            //
            let st = idx;
            let ed = idx + ss.len();
            for m in line_color_mark.iter_mut().take(ed).skip(st) {
                *m = true;
            }
        }
    }
    Ok((false, b_found, line_color_mark))
}

fn make_out_s(
    color_start_s: &str,
    color_end_s: &str,
    line_ss: &str,
    line_color_mark: &[bool],
) -> anyhow::Result<String> {
    let line_len: usize = line_ss.len();
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
    Ok(out_s)
}
