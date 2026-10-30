use std::{collections::HashMap, env, io::Result, process::Command};

use clap::Parser;

use crate::{build_file::build_file_compile, helper::make_dir, tui::start_tui};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value_t = false, help = "Start TUI")]
    tui: bool,
    #[arg(short, long, default_value_t = ".".to_string(), help = "Path to project root")]
    path: String,
    #[arg(short, long, default_value_t = false, help = "Build stripped binary")]
    release: bool,
    #[arg(short, long, default_value_t = "".to_string(), help = "Use a build file")]
    build: String,
    #[arg(short = 'B', long, default_value_t = "".to_string(), help = "Create binary project template")]
    bin: String,
    #[arg(short = 'L', long, default_value_t = "".to_string(), help = "Create library project template")]
    lib: String,
}

pub fn check_args() -> Result<()> {
    let args = Args::parse();
    if args.tui {
        start_tui()?;
        return Ok(());
    } else {
        if args.bin.trim() != "" || args.lib.trim() != "" {
            if args.bin.trim() != "" {
                make_dir(&args.bin, true)?;
                return Ok(());
            } else {
                make_dir(&args.lib, false)?;
                return Ok(());
            }
        }

        if args.build.trim() != "" {
            let (sout, serr, ec) = build_file_compile(args.build, args.path)?;
            if ec != 0 {
                eprintln!("{}", serr);
                return Ok(());
            } else {
                println!("{}", sout);
                return Ok(());
            }
        }

        let env: HashMap<String, String> = env::vars().collect();

        if args.release {
            let _ = Command::new("go")
                .args(["build", "-ldflags", "-s -w", "-trimpath", &args.path])
                .envs(&env)
                .status()?;
        } else {
            let _ = Command::new("go")
                .args(["build", &args.path])
                .envs(&env)
                .status()?;
        }
    }

    Ok(())
}
