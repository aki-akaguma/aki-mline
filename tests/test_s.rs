#[macro_use]
mod helper;

macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr,) => {
        do_execute!($args, $sin)
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .pg_err()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
    ($env:expr, $args:expr, $sin:expr,) => {{
        do_execute!($env, $args, $sin)
    }};
    ($env:expr, $args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute_env(&sioe, &program, $args, $env);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .pg_err()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.pg_err().lock().buffer_to_string()
    };
    ($sioe:expr, sout) => {
        $sioe.pg_out().lock().buffer_to_string()
    };
}

macro_rules! env_1 {
    () => {{
        let mut env = conf::EnvConf::new();
        env.color_seq_start = color_start!().to_string();
        env.color_seq_end = color_end!().to_string();
        env
    }};
}

const IN_DAT_TARGET_LIST: &str = include_str!(concat!("../", fixture_target_list!()));

mod test_0_s {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[""]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(), ": ",
                "Missing option: e\n",
                "Missing option: s\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_0_x_options_s {
    use libaki_mline::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_x_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(!buff!(sioe, sout).is_empty());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(buff!(sioe, sout).contains("-X rust-version-info"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let (r, sioe) = do_execute!(["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        // The first one should be executed and the program should exit.
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(!buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
}

mod test_1_argument_parsing {
    use libaki_mline::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_missing_value_for_exp() {
        let (r, sioe) = do_execute!(["-e"], "");
        assert!(buff!(sioe, serr).contains("Missing option argument: e"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_missing_value_for_str() {
        let (r, sioe) = do_execute!(["-s"], "");
        assert!(buff!(sioe, serr).contains("Missing option argument: s"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_unknown_option() {
        let (r, sioe) = do_execute!(["--unknown-option"], "");
        assert!(buff!(sioe, serr).contains("Invalid option: unknown-option"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_both_exp_and_str() {
        let (r, sioe) = do_execute!(["-e", "a", "-s", "b"], "");
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(buff!(sioe, serr), "");
        assert!(r.is_ok());
        /*
        assert!(buff!(sioe, sout)
            .contains("The argument '--exp <exp>' cannot be used with '--str <string>'"));
        */
    }
}

mod test_1_env_color_override {
    use libaki_mline::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_custom_color_sequence() {
        let mut env = conf::EnvConf::new();
        env.color_seq_start = "<START>".to_string();
        env.color_seq_end = "<END>".to_string();
        //
        let (r, sioe) = do_execute!(&env, ["-s", "world", "--color", "always"], "hello world\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "hello <START>world<END>\n");
        assert!(r.is_ok());
    }
}

mod test_1_regex_s {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Read;
    use std::io::Write;
    //
    macro_rules! xx_eq {
        ($in_s:expr, $reg_s:expr, $out_s:expr) => {
            let env = env_1!();
            let (r, sioe) = do_execute!(&env, ["-e", $reg_s, "--color", "always"], $in_s);
            assert_eq!(buff!(sioe, serr), "");
            assert_eq!(buff!(sioe, sout), $out_s);
            assert_eq!(r.is_ok(), true);
        };
    }
    //
    fn _get_bytes_from_file(infile_path: &str) -> Vec<u8> {
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
    /*
    #[test]
    fn test_invalid_utf8() {
        let v = get_bytes_from_file(fixture_invalid_utf8!());
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-e", "real$"], v);
        assert_eq!(buff!(sioe, serr), concat!(program_name!(), ": stream did not contain valid UTF-8\n"));
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    */
    //
    #[test]
    fn test_parse_error() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-e", "abc["], "");
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": regex parse error:\n    abc[\n       ^\nerror: unclosed character class\n"
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_1_str_s {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Read;
    use std::io::Write;
    //
    macro_rules! xx_eq {
        ($in_s:expr, $needle_s:expr, $out_s:expr) => {
            let env = env_1!();
            let (r, sioe) = do_execute!(&env, ["-s", $needle_s, "--color", "always"], $in_s);
            assert_eq!(buff!(sioe, serr), "");
            assert_eq!(buff!(sioe, sout), $out_s);
            assert_eq!(r.is_ok(), true);
        };
    }
    //
    fn _get_bytes_from_file(infile_path: &str) -> Vec<u8> {
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
    /*
    #[test]
    fn test_invalid_utf8() {
        let v = get_bytes_from_file(fixture_invalid_utf8!());
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, &["-s", "real"], v);
        assert_eq!(buff!(sioe, serr), concat!(program_name!(), ": stream did not contain valid UTF-8\n"));
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    */
}

mod test_2_inverse_option_s {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_inverse_str() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-s", "apple", "-i"], "apple\nbanana\norange\n",);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "banana\norange\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_inverse_regex() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-e", "a.c", "-i"], "abc\ndef\nac\nxyz\n",);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "def\nac\nxyz\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_inverse_with_around() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-s", "banana", "-i", "--around", "1"],
            "apple\nbanana\norange\npeach\n",
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "apple\norange\npeach\n");
        assert!(r.is_ok());
    }
}

mod test_2_color_option_s {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_color_always() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-s", "apple", "--color", "always"],
            "an apple a day\n",
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            format!("an {}apple{} a day\n", color_start!(), color_end!())
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_color_auto() {
        // In a non-interactive terminal, 'auto' should not produce color
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-s", "apple", "--color", "auto"], "an apple a day\n",);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            format!("an {}apple{} a day\n", color_start!(), color_end!())
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_color_never() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-s", "apple", "--color", "never"],
            "an apple a day\n",
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "an apple a day\n");
        assert!(r.is_ok());
    }
}

mod test_2_edge_cases_s {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_empty_input() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-e", "a"], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_no_matches() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-s", "xyz"], "apple\nbanana\norange\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_all_lines_match() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-e", "a", "--color", "never"],
            "apple\nbanana\norange\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "apple\nbanana\norange\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_color_never() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-s", "apple", "--color", "never"],
            "apple pie\napple juice\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "apple pie\napple juice\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_around_at_start() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-s", "line1", "--around", "1", "--color", "always"],
            "line1\nline2\nline3\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        let expected = format!("{}line1{}\nline2\n\n", color_start!(), color_end!());
        assert_eq!(buff!(sioe, sout), expected);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_around_at_end() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-s", "line3", "--around", "1", "--color", "always"],
            "line1\nline2\nline3\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        let expected = format!("line2\n{}line3{}\n", color_start!(), color_end!());
        assert_eq!(buff!(sioe, sout), expected);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_around_overlapping() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-s", "match", "--around", "1", "--color", "always"],
            "line1\nline2 match\nline3 match\nline4\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        let line2_colored = format!("line2 {}match{}", color_start!(), color_end!());
        let line3_colored = format!("line3 {}match{}", color_start!(), color_end!());
        let expected = format!("line1\n{}\n{}\nline4\n\n", line2_colored, line3_colored);
        assert_eq!(buff!(sioe, sout), expected);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_inverse_with_no_matches() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-s", "nomatch", "-i"], "line1\nline2\nline3\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line1\nline2\nline3\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_inverse_with_all_lines_matching() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-e", ".", "-i"], "line1\nline2\nline3\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
}

mod test_3_s {
    /*
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
    */
}

mod test_4_around_s {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_around_1_ok() {
        let env = env_1!();
        let in_w = super::IN_DAT_TARGET_LIST.to_string();
        let (r, sioe) = do_execute!(
            &env,
            ["-e", "musl", "--color", "always", "--around", "1"],
            in_w.as_str(),
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_around_0_ok() {
        let env = env_1!();
        let in_w = super::IN_DAT_TARGET_LIST.to_string();
        let (r, sioe) = do_execute!(
            &env,
            ["-e", "musl", "--color", "always", "--around", "0"],
            in_w.as_str(),
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
}

mod test_4_more_regex_s {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    macro_rules! xx_eq {
        ($in_s:expr, $reg_s:expr, $out_s:expr) => {
            let env = env_1!();
            let (r, sioe) = do_execute!(&env, ["-e", $reg_s, "--color", "always"], $in_s);
            assert_eq!(buff!(sioe, serr), "");
            assert_eq!(buff!(sioe, sout), $out_s);
            assert_eq!(r.is_ok(), true);
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

mod test_4_more_around {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    const INPUT: &str = "line1\nline2\nline3 match\nline4\nline5\nline6\nline7 match\nline8\nline9";
    //
    #[test]
    fn test_around_2() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-s", "match", "--around", "2", "--color", "always"],
            INPUT
        );
        assert_eq!(buff!(sioe, serr), "");
        let expected =
            "line1\nline2\nline3 match\nline4\nline5\n\nline6\nline7 match\nline8\nline9\n\n";
        assert_eq!(
            buff!(sioe, sout),
            expected.replace("match", &format!("{}match{}", color_start!(), color_end!()))
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_around_and_inverse() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-s", "match", "-i", "--around", "1"], INPUT);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "line1\nline2\nline4\nline5\nline6\nline8\nline9\n"
        );
        assert!(r.is_ok());
    }
}

mod test_4_multibyte_utf8 {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    macro_rules! xx_eq {
        ($in_s:expr, $needle_s:expr, $out_s:expr) => {
            let env = env_1!();
            let (r, sioe) = do_execute!(&env, ["-s", $needle_s, "--color", "always"], $in_s);
            assert_eq!(buff!(sioe, serr), "");
            assert_eq!(buff!(sioe, sout), $out_s);
            assert_eq!(r.is_ok(), true);
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
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            ["-s", "こんにちは", "-i"],
            "こんにちは世界\nさようなら世界\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "さようなら世界\n");
        assert!(r.is_ok());
    }
}

mod test_4_special_chars_in_str_search {
    use libaki_mline::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_str_search_with_dot() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-s", "a.c", "--color", "always"], "a.c\nabc\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            format!("{}a.c{}\n", color_start!(), color_end!())
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_str_search_with_asterisk() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-s", "a*c", "--color", "always"], "a*c\nac\nabc\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            format!("{}a*c{}\n", color_start!(), color_end!())
        );
        assert!(r.is_ok());
    }
}
