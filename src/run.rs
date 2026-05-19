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

struct MatchResult {
    b_continue: bool,
    b_found: bool,
    ranges: Vec<(usize, usize)>,
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
        let res = make_line_color_mark(regs, &conf.opt_str, conf.flg_inverse, line_ss)?;
        if res.b_continue {
            continue 'line_get;
        }
        if conf.flg_inverse && !res.b_found {
            sioe.pg_out().write_line(line_s)?;
            continue;
        } else if res.b_found {
            if !prevs.is_empty() {
                prevs.reverse();
                while let Some(line) = prevs.pop() {
                    sioe.pg_out().write_line(line)?;
                }
            }
            prev_found = true;
            //
            let out_s = if let OptColorWhen::Always = conf.opt_color {
                make_out_s(color_start_s, color_end_s, line_ss, &res.ranges)?
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
) -> anyhow::Result<MatchResult> {
    use naive_opt::Search;
    let mut matches: Vec<(usize, usize)> = Vec::new();
    let mut b_found = false;
    //
    for re in regs {
        for mat in re.find_iter(line_ss) {
            b_found = true;
            if flg_inverse {
                return Ok(MatchResult {
                    b_continue: true,
                    b_found,
                    ranges: Vec::new(),
                });
            };
            matches.push((mat.start(), mat.end()));
        }
    }
    for needle in opt_str.iter() {
        for (idx, ss) in line_ss.search_indices(needle) {
            b_found = true;
            if flg_inverse {
                return Ok(MatchResult {
                    b_continue: true,
                    b_found,
                    ranges: Vec::new(),
                });
            };
            matches.push((idx, idx + ss.len()));
        }
    }
    //
    if !b_found || flg_inverse {
        return Ok(MatchResult {
            b_continue: false,
            b_found,
            ranges: Vec::new(),
        });
    }
    //
    // Merge ranges
    matches.sort_unstable_by_key(|m| m.0);
    let mut merged = Vec::with_capacity(matches.len());
    if let Some(first) = matches.first() {
        let mut current = *first;
        for next in matches.iter().skip(1) {
            if next.0 <= current.1 {
                current.1 = current.1.max(next.1);
            } else {
                merged.push(current);
                current = *next;
            }
        }
        merged.push(current);
    }
    //
    Ok(MatchResult {
        b_continue: false,
        b_found,
        ranges: merged,
    })
}

fn make_out_s(
    color_start_s: &str,
    color_end_s: &str,
    line_ss: &str,
    line_color_ranges: &[(usize, usize)],
) -> anyhow::Result<String> {
    let mut out_s = String::with_capacity(
        line_ss.len() + line_color_ranges.len() * (color_start_s.len() + color_end_s.len()),
    );
    let mut last_pos = 0;
    for (st, ed) in line_color_ranges {
        out_s.push_str(&line_ss[last_pos..*st]);
        out_s.push_str(color_start_s);
        out_s.push_str(&line_ss[*st..*ed]);
        out_s.push_str(color_end_s);
        last_pos = *ed;
    }
    out_s.push_str(&line_ss[last_pos..]);
    Ok(out_s)
}
