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
            let (r, sioe) = do_execute!(&env, &["-e", $reg_s, "--color", "always"], $in_s);
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
        let (r, sioe) = do_execute!(&env, &["-e", "real$"], v);
        assert_eq!(buff!(sioe, serr), concat!(program_name!(), ": stream did not contain valid UTF-8\n"));
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    */
    //
    #[test]
    fn test_parse_error() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, &["-e", "abc["], "");
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
            let (r, sioe) = do_execute!(&env, &["-s", $needle_s, "--color", "always"], $in_s);
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

mod test_3 {
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
            &["-e", "musl", "--color", "always", "--around", "1"],
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
            &["-e", "musl", "--color", "always", "--around", "0"],
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
