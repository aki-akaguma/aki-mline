const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

#[macro_use]
mod helper_e;

mod test_0_e {
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
}

mod test_0_x_options_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, x_help_msg!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        // The first one should be executed and the program should exit.
        assert!(oup.stdout.contains("Options:"));
        assert!(!oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
}

mod test_1_argument_parsing_e {
    use exec_target::exec_target;
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_missing_value_for_exp() {
        let oup = exec_target(TARGET_EXE_PATH, ["-e"]);
        assert!(oup.stderr.contains("Missing option argument: e"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_missing_value_for_str() {
        let oup = exec_target(TARGET_EXE_PATH, ["-s"]);
        assert!(oup.stderr.contains("Missing option argument: s"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_unknown_option() {
        let oup = exec_target(TARGET_EXE_PATH, ["--unknown-option"]);
        assert!(oup.stderr.contains("Invalid option: unknown-option"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_both_exp_and_str() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "a", "-s", "b"], b"");
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.stderr, "");
        /*
        assert!(oup
            .stderr
            .contains("The argument '--exp <exp>' cannot be used with '--str <string>'"));
        */
        assert!(oup.status.success());
    }
}

mod test_1_env_color_override_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_custom_color_sequence() {
        let mut env = env_1!();
        env.push(("AKI_MLINE_COLOR_SEQ_ST", "<START>"));
        env.push(("AKI_MLINE_COLOR_SEQ_ED", "<END>"));
        //
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "world", "--color", "always"],
            env,
            "hello world\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "hello <START>world<END>\n");
        assert!(oup.status.success());
    }
}

mod test_1_regex_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    use std::io::Read;
    //
    macro_rules! xx_eq {
        ($in_s:expr, $reg_s:expr, $out_s:expr) => {
            let oup = exec_target_with_env_in(
                TARGET_EXE_PATH,
                ["-e", $reg_s, "--color", "always"],
                env_1!(),
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

mod test_1_str_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    use std::io::Read;
    //
    macro_rules! xx_eq {
        ($in_s:expr, $needle_s:expr, $out_s:expr) => {
            let oup = exec_target_with_env_in(
                TARGET_EXE_PATH,
                ["-s", $needle_s, "--color", "always"],
                env_1!(),
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

mod test_2_inverse_option_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_inverse_str() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "apple", "-i"],
            env_1!(),
            "apple\nbanana\norange\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "banana\norange\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_inverse_regex() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "a.c", "-i"],
            env_1!(),
            "abc\ndef\nac\nxyz\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "def\nac\nxyz\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_inverse_with_around() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "banana", "-i", "--around", "1"],
            env_1!(),
            "apple\nbanana\norange\npeach\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "apple\norange\npeach\n");
        assert!(oup.status.success());
    }
}

mod test_2_color_option_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_color_always() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "apple", "--color", "always"],
            env_1!(),
            "an apple a day\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            format!("an {}apple{} a day\n", color_start!(), color_end!())
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_color_auto() {
        // In a non-interactive terminal, 'auto' should not produce color
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "apple", "--color", "auto"],
            env_1!(),
            "an apple a day\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "an apple a day\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_color_never() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "apple", "--color", "never"],
            env_1!(),
            "an apple a day\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "an apple a day\n");
        assert!(oup.status.success());
    }
}

mod test_2_edge_cases_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_empty_input() {
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-e", "a"], env_1!(), "".as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_no_matches() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "xyz"],
            env_1!(),
            "apple\nbanana\norange\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_all_lines_match() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", ".", "--color", "never"],
            env_1!(),
            "apple\nbanana\norange\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "apple\nbanana\norange\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_color_never() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "apple", "--color", "never"],
            env_1!(),
            "apple pie\napple juice\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "apple pie\napple juice\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_around_at_start() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "line1", "--around", "1", "--color", "always"],
            env_1!(),
            "line1\nline2\nline3\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        let expected = format!("{}line1{}\nline2\n\n", color_start!(), color_end!());
        assert_eq!(oup.stdout, expected);
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_around_at_end() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "line3", "--around", "1", "--color", "always"],
            env_1!(),
            "line1\nline2\nline3\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        let expected = format!("line2\n{}line3{}\n", color_start!(), color_end!());
        assert_eq!(oup.stdout, expected);
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_around_overlapping() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "match", "--around", "1", "--color", "always"],
            env_1!(),
            "line1\nline2 match\nline3 match\nline4\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        let line2_colored = format!("line2 {}match{}", color_start!(), color_end!());
        let line3_colored = format!("line3 {}match{}", color_start!(), color_end!());
        let expected = format!("line1\n{}\n{}\nline4\n\n", line2_colored, line3_colored);
        assert_eq!(oup.stdout, expected);
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_inverse_with_no_matches() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "nomatch", "-i"],
            env_1!(),
            "line1\nline2\nline3\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line1\nline2\nline3\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_inverse_with_all_lines_matching() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", ".", "-i"],
            env_1!(),
            "line1\nline2\nline3\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
}

mod test_3_e {
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

mod test_4_around_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    use std::io::Read;
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
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "musl", "--color", "always", "--around", "1"],
            env_1!(),
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
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "musl", "--color", "always", "--around", "0"],
            env_1!(),
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

mod test_4_more_regex_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    macro_rules! xx_eq {
        ($in_s:expr, $reg_s:expr, $out_s:expr) => {
            let oup = exec_target_with_env_in(
                TARGET_EXE_PATH,
                ["-e", $reg_s, "--color", "always"],
                env_1!(),
                $in_s.as_bytes(),
            );
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, $out_s);
            assert_eq!(oup.status.success(), true);
        };
    }
    //
    #[test]
    fn test_char_classes() {
        xx_eq!(
            "line 1\nline 2\nline a",
            r"line \d",
            format!(
                "{}line 1{}\n{}line 2{}\n",
                color_start!(),
                color_end!(),
                color_start!(),
                color_end!()
            )
        );
        xx_eq!(
            "word1\nword2\n word3",
            r"\w+",
            format!(
                "{}word1{}\n{}word2{}\n {}word3{}\n",
                color_start!(),
                color_end!(),
                color_start!(),
                color_end!(),
                color_start!(),
                color_end!()
            )
        );
    }
    //
    #[test]
    fn test_quantifiers() {
        xx_eq!(
            "ab\nac\nabc",
            "ab?c",
            format!(
                "{}ac{}\n{}abc{}\n",
                color_start!(),
                color_end!(),
                color_start!(),
                color_end!()
            )
        );
        xx_eq!(
            "ac\nabc\nabbc",
            "ab*c",
            format!(
                "{}ac{}\n{}abc{}\n{}abbc{}\n",
                color_start!(),
                color_end!(),
                color_start!(),
                color_end!(),
                color_start!(),
                color_end!()
            )
        );
        xx_eq!(
            "ac\nabc\nabbc",
            "ab+c",
            format!(
                "{}abc{}\n{}abbc{}\n",
                color_start!(),
                color_end!(),
                color_start!(),
                color_end!()
            )
        );
    }
}

mod test_4_more_around_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    const INPUT: &str = "line1\nline2\nline3 match\nline4\nline5\nline6\nline7 match\nline8\nline9";
    //
    #[test]
    fn test_around_2() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "match", "--around", "2", "--color", "always"],
            env_1!(),
            INPUT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        let expected =
            "line1\nline2\nline3 match\nline4\nline5\n\nline6\nline7 match\nline8\nline9\n\n";
        assert_eq!(
            oup.stdout,
            expected.replace("match", &format!("{}match{}", color_start!(), color_end!()))
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_around_and_inverse() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "match", "-i", "--around", "1"],
            env_1!(),
            INPUT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "line1\nline2\nline4\nline5\nline6\nline8\nline9\n"
        );
        assert!(oup.status.success());
    }
}

mod test_4_multibyte_utf8_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    macro_rules! xx_eq {
        ($in_s:expr, $needle_s:expr, $out_s:expr) => {
            let oup = exec_target_with_env_in(
                TARGET_EXE_PATH,
                ["-s", $needle_s, "--color", "always"],
                env_1!(),
                $in_s.as_bytes(),
            );
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, $out_s);
            assert_eq!(oup.status.success(), true);
        };
    }
    //
    #[test]
    fn test_multibyte_char_match() {
        xx_eq!(
            "こんにちは世界\nさようなら世界",
            "世界",
            format!(
                "こんにちは{0}世界{1}\nさようなら{0}世界{1}\n",
                color_start!(),
                color_end!()
            )
        );
    }
    //
    #[test]
    fn test_multibyte_char_no_match() {
        xx_eq!("こんにちは\nさようなら", "世界", "");
    }
    //
    #[test]
    fn test_inverse_multibyte() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "こんにちは", "-i"],
            env_1!(),
            "こんにちは世界\nさようなら世界\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "さようなら世界\n");
        assert!(oup.status.success());
    }
}

mod test_4_special_chars_in_str_search_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_str_search_with_dot() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "a.c", "--color", "always"],
            env_1!(),
            "a.c\nabc\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            format!("{}a.c{}\n", color_start!(), color_end!())
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_str_search_with_asterisk() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-s", "a*c", "--color", "always"],
            env_1!(),
            "a*c\nac\nabc\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            format!("{}a*c{}\n", color_start!(), color_end!())
        );
        assert!(oup.status.success());
    }
}
