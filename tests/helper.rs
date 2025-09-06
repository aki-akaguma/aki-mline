#[allow(unused_macros)]
macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
            Usage:
              aki-mline [options]

            match line, regex text filter like a grep.

            Options:
                  --around <num>    around output. printing the match, prev and the next lines.
                  --color <when>    use markers to highlight the matching strings
              -e, --exp <exp>       regular expression
              -s, --str <string>    simple string match
              -i, --inverse         output non-matching lines.

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

            Option Parameters:
              <when>    'always', 'never', or 'auto'
              <exp>     regular expression
              <string>  simple string, non regular expression

            Environments:
              AKI_MLINE_COLOR_SEQ_ST    color start sequence specified by ansi
              AKI_MLINE_COLOR_SEQ_ED    color end sequence specified by ansi
            "#
            ),
            "\n",
        )
    };
}

#[allow(unused_macros)]
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

#[allow(unused_macros)]
macro_rules! program_name {
    () => {
        "aki-mline"
    };
}

#[allow(unused_macros)]
macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

#[allow(unused_macros)]
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_target_list {
    () => {
        "fixtures/rustup-target-list.txt"
    };
}

//
#[allow(unused_macros)]
macro_rules! color_start {
    //() => { "\u{1B}[01;31m" }
    () => {
        "<S>"
    };
}

#[allow(unused_macros)]
macro_rules! color_end {
    //() => {"\u{1B}[0m"}
    () => {
        "<E>"
    };
}
