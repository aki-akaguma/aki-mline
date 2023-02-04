const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

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

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "aki-mline"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}

macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

macro_rules! fixture_target_list {
    () => {
        "fixtures/rustup-target-list.txt"
    };
}

//mod helper;

mod test_0 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_non_opt_non_arg() {
        let oup = exec_target(TARGET_EXE_PATH, [""]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: e\n",
                "Missing option: s\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
} // mod test_0

mod test_regex {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    use std::collections::HashMap;
    use std::io::Read;
    //
    macro_rules! color_start {
        //() => { "\u{1B}[01;31m" }
        () => {
            "<S>"
        };
    }
    macro_rules! color_end {
        //() => {"\u{1B}[0m"}
        () => {
            "<E>"
        };
    }
    macro_rules! env_1 {
        () => {{
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert(
                "AKI_MLINE_COLOR_SEQ_ST".to_string(),
                color_start!().to_string(),
            );
            env.insert(
                "AKI_MLINE_COLOR_SEQ_ED".to_string(),
                color_end!().to_string(),
            );
            env
        }};
    }
    macro_rules! xx_eq {
        ($in_s:expr, $reg_s:expr, $out_s:expr) => {
            let env = env_1!();
            let oup = exec_target_with_env_in(
                TARGET_EXE_PATH,
                &["-e", $reg_s, "--color", "always"],
                env,
                $in_s.as_bytes(),
            );
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, $out_s);
            assert_eq!(oup.status.success(), true);
        };
    }
    //
    fn get_bytes_from_file(infile_path: &str) -> Vec<u8> {
        let mut f = std::fs::File::open(infile_path).unwrap();
        let mut v: Vec<u8> = Vec::new();
        f.read_to_end(&mut v).unwrap();
        v
    }
    //
    #[test]
    fn test_ok() {
        xx_eq!(
            "The cat sat in the hat",
            "[csh].. [csh].. in",
            concat!(
                "The ",
                color_start!(),
                "cat sat in",
                color_end!(),
                " the hat",
                "\n",
            )
        );
        //
        // alternation operator : pat|abc
        xx_eq!(
            "Feliformia",
            "and|or",
            concat!("Felif", color_start!(), "or", color_end!(), "mia", "\n",)
        );
        xx_eq!(
            "furandi",
            "and|or",
            concat!("fur", color_start!(), "and", color_end!(), "i", "\n",)
        );
        xx_eq!("dissemblance", "and|or", "");
        //
        // anchor
        //// line head
        xx_eq!(
            "surrealist",
            "^sur",
            concat!("", color_start!(), "sur", color_end!(), "realist", "\n",)
        );
        xx_eq!("surrealist", "^real", "");
        //// line tail
        xx_eq!(
            "surrealist",
            "list$",
            concat!("surrea", color_start!(), "list", color_end!(), "\n")
        );
        xx_eq!("surrealist", "real$", "");
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let v = get_bytes_from_file(fixture_invalid_utf8!());
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-e", "real$"], env, v.as_slice());
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n")
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_parse_error() {
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-e", "abc["], env, "".as_bytes());
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": regex parse error:\n    abc[\n       ^\nerror: unclosed character class\n"
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_str {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    use std::collections::HashMap;
    use std::io::Read;
    //
    macro_rules! color_start {
        //() => { "\u{1B}[01;31m" }
        () => {
            "<S>"
        };
    }
    macro_rules! color_end {
        //() => {"\u{1B}[0m"}
        () => {
            "<E>"
        };
    }
    macro_rules! env_1 {
        () => {{
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert(
                "AKI_MLINE_COLOR_SEQ_ST".to_string(),
                color_start!().to_string(),
            );
            env.insert(
                "AKI_MLINE_COLOR_SEQ_ED".to_string(),
                color_end!().to_string(),
            );
            env
        }};
    }
    macro_rules! xx_eq {
        ($in_s:expr, $needle_s:expr, $out_s:expr) => {
            let env = env_1!();
            let oup = exec_target_with_env_in(
                TARGET_EXE_PATH,
                &["-s", $needle_s, "--color", "always"],
                env,
                $in_s.as_bytes(),
            );
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, $out_s);
            assert_eq!(oup.status.success(), true);
        };
    }
    //
    fn get_bytes_from_file(infile_path: &str) -> Vec<u8> {
        let mut f = std::fs::File::open(infile_path).unwrap();
        let mut v: Vec<u8> = Vec::new();
        f.read_to_end(&mut v).unwrap();
        v
    }
    //
    #[test]
    fn test_ok() {
        xx_eq!(
            "The cat sat in the hat",
            "cat sat in",
            concat!(
                "The ",
                color_start!(),
                "cat sat in",
                color_end!(),
                " the hat",
                "\n",
            )
        );
        //
        // alternation operator : pat|abc
        xx_eq!(
            "Feliformia",
            "or",
            concat!("Felif", color_start!(), "or", color_end!(), "mia", "\n",)
        );
        xx_eq!(
            "furandi",
            "and",
            concat!("fur", color_start!(), "and", color_end!(), "i", "\n",)
        );
        xx_eq!("dissemblance", "and", "");
        //
        // anchor
        //// line head
        xx_eq!(
            "surrealist",
            "sur",
            concat!("", color_start!(), "sur", color_end!(), "realist", "\n",)
        );
        //xx_eq!("surrealist", "real", "");
        //// line tail
        xx_eq!(
            "surrealist",
            "list",
            concat!("surrea", color_start!(), "list", color_end!(), "\n")
        );
        //xx_eq!("surrealist", "real", "");
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let v = get_bytes_from_file(fixture_invalid_utf8!());
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-s", "real"], env, v.as_slice());
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n")
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_3 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "cat \"{}\" | \"{}\" -e \".\" | head -n 2",
            fixture_text10k!(),
            TARGET_EXE_PATH
        );
        let oup = exec_target("sh", ["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ABCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
}

mod test_around {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    use std::collections::HashMap;
    use std::io::Read;
    //
    macro_rules! env_1 {
        () => {{
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert("AKI_MLINE_COLOR_SEQ_ST".to_string(), "<S>".to_string());
            env.insert("AKI_MLINE_COLOR_SEQ_ED".to_string(), "<E>".to_string());
            env
        }};
    }
    //
    fn get_bytes_from_file(infile_path: &str) -> Vec<u8> {
        let mut f = std::fs::File::open(infile_path).unwrap();
        let mut v: Vec<u8> = Vec::new();
        f.read_to_end(&mut v).unwrap();
        v
    }
    //
    #[test]
    fn test_around_1_ok() {
        let v = get_bytes_from_file(fixture_target_list!());
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "musl", "--color", "always", "--around", "1"],
            env,
            v.as_slice(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "aarch64-unknown-linux-gnu (installed)\n",
                "aarch64-unknown-linux-<S>musl<E> (installed)\n",
                "aarch64-unknown-none\n",
                "\n",
                "arm-unknown-linux-gnueabihf\n",
                "arm-unknown-linux-<S>musl<E>eabi\n",
                "arm-unknown-linux-<S>musl<E>eabihf\n",
                "armebv7r-none-eabi\n",
                "\n",
                "armv5te-unknown-linux-gnueabi\n",
                "armv5te-unknown-linux-<S>musl<E>eabi\n",
                "armv7-linux-androideabi\n",
                "\n",
                "armv7-unknown-linux-gnueabihf (installed)\n",
                "armv7-unknown-linux-<S>musl<E>eabi\n",
                "armv7-unknown-linux-<S>musl<E>eabihf (installed)\n",
                "armv7a-none-eabi\n",
                "\n",
                "i586-unknown-linux-gnu\n",
                "i586-unknown-linux-<S>musl<E>\n",
                "i686-linux-android\n",
                "\n",
                "i686-unknown-linux-gnu (installed)\n",
                "i686-unknown-linux-<S>musl<E> (installed)\n",
                "mips-unknown-linux-gnu\n",
                "\n",
                "mips-unknown-linux-<S>musl<E>\n",
                "mips64-unknown-linux-gnuabi64\n",
                "\n",
                "mips64-unknown-linux-<S>musl<E>abi64\n",
                "mips64el-unknown-linux-gnuabi64 (installed)\n",
                "\n",
                "mips64el-unknown-linux-<S>musl<E>abi64 (installed)\n",
                "mipsel-unknown-linux-gnu (installed)\n",
                "\n",
                "mipsel-unknown-linux-<S>musl<E> (installed)\n",
                "nvptx64-nvidia-cuda\n",
                "\n",
                "x86_64-unknown-linux-gnux32\n",
                "x86_64-unknown-linux-<S>musl<E> (installed)\n",
                "x86_64-unknown-netbsd\n",
                "\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_around_0_ok() {
        let v = get_bytes_from_file(fixture_target_list!());
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "musl", "--color", "always", "--around", "0"],
            env,
            v.as_slice(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "aarch64-unknown-linux-<S>musl<E> (installed)\n",
                "arm-unknown-linux-<S>musl<E>eabi\n",
                "arm-unknown-linux-<S>musl<E>eabihf\n",
                "armv5te-unknown-linux-<S>musl<E>eabi\n",
                "armv7-unknown-linux-<S>musl<E>eabi\n",
                "armv7-unknown-linux-<S>musl<E>eabihf (installed)\n",
                "i586-unknown-linux-<S>musl<E>\n",
                "i686-unknown-linux-<S>musl<E> (installed)\n",
                "mips-unknown-linux-<S>musl<E>\n",
                "mips64-unknown-linux-<S>musl<E>abi64\n",
                "mips64el-unknown-linux-<S>musl<E>abi64 (installed)\n",
                "mipsel-unknown-linux-<S>musl<E> (installed)\n",
                "x86_64-unknown-linux-<S>musl<E> (installed)\n",
            )
        );
        assert!(oup.status.success());
    }
}
