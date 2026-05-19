use std::fs;

const DESCRIPTIONS_TEXT: &str = r#"match line, regex text filter like a grep."#;
const PARAMS_TEXT: &str = r#"Option Parameters:
  <when>    'always', 'never', or 'auto'
  <exp>     regular expression
  <string>  simple string, non regular expression"#;
const ENV_TEXT: &str = r#"Environments:
  AKI_MLINE_COLOR_SEQ_ST    color start sequence specified by ansi
  AKI_MLINE_COLOR_SEQ_ED    color end sequence specified by ansi"#;

pub fn do_update_docs() -> anyhow::Result<()> {
    let options_text = fs::read_to_string("xtask/src/aki-mline-cmd.txt")?;

    let help_message = format!(
        "```text\nUsage:\n  aki-mline [options]\n\n{}\n\n{}\n\n{}\n\n{}\n```\n",
        DESCRIPTIONS_TEXT,
        options_text.trim_end(),
        PARAMS_TEXT,
        ENV_TEXT
    );

    update_file("src/lib.rs", &help_message)?;

    Ok(())
}

fn update_file(path: &str, help_msg: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(path)?;
    let start_marker = "<!-- [HELP_START] -->\n";
    let end_marker = "<!-- [HELP_END] -->";

    if let Some(start_pos) = content.find(start_marker) {
        if let Some(end_pos) = content.find(end_marker) {
            let mut new_content = String::new();
            new_content.push_str(&content[..start_pos + start_marker.len()]);
            new_content.push_str(help_msg);
            new_content.push_str(&content[end_pos..]);

            fs::write(path, new_content)?;
            println!("Updated {path}");
        }
    }
    Ok(())
}
