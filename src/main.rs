//  Brainfck compiler
//  Copyright (C) 2025  František Slivko <slivko.frantisek@gmail.com>
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.




#[allow(unused)]
use std::fmt::Write as _;
#[allow(unused)]
use std::io::Write as _;
#[allow(unused)]
use std::io::Read;
use std::ops::Not;
use std::path::{PathBuf};
#[allow(unused)]
use clap::{arg, command, crate_authors, value_parser, ArgMatches};
use clap::{Arg, ArgAction};
use clap::builder::{PathBufValueParser};

macro_rules! write {
  ($dst:expr, $($arg:tt)*) => { _ = std::write!($dst, $($arg)*); }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
    PrintNumber,
}

impl Token {
    fn from_char(c: &char) -> Option<Self> {
        Some(match c {
            '>' => Token::Right,
            '<' => Token::Left,
            '+' => Token::Increment,
            '-' => Token::Decrement,
            '.' => Token::Output,
            ',' => Token::Input,
            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,
            '!' => Token::PrintNumber,
            _ => { return None; }
        })
    }
}

const PREABLE: &'static str = include_str!("head.ll");

#[derive(Debug)]
struct SettingsArgs {
    llvm_emit: Option<PathBuf>,
    run_clang: bool,
    output: PathBuf,
    cells_count: u16,
    override_new_line_to_null: bool,
}

#[derive(Debug)]
struct CompileArgs {
    source: PathBuf,
}

#[derive(Debug)]
enum Args {
    Compile { code: String, ca: CompileArgs, sa: SettingsArgs },
    Repl { code: String, sa: SettingsArgs },
}

/// Err(failed?)
fn process_args() -> Result<Args, bool> {
    let mut cmd = clap::Command::new("brainfck compiler")
        .bin_name("bf")
        .version("0.1.0")
        // .author(crate_authors!(", "))
        .help_template("{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")
        .subcommand(command!("compile").about("Compiles a source file")
            .disable_version_flag(true)
            .arg(Arg::new("FL")
                .required(true)
                .action(ArgAction::Set)
                .value_name("source")
                .help("Source file (file containing brainfck program)"))
            .arg(Arg::new("ON")
                .short('o')
                .long("output")
                .value_name("file")
                .required(false)
                .action(ArgAction::Set)
                .default_value({
                    #[cfg(windows)]
                    { "out.exe" }
                    #[cfg(not(windows))]
                    { "out" }
                })
                .help("Specifies output filename"))
            .arg(Arg::new("CC")
                .short('c')
                .long("cell-count")
                .value_name("count")
                .action(ArgAction::Set)
                .required(false)
                .default_value("30000")
                .value_parser(value_parser!(u16))
                .help("Specifies how many cells should there be"))
            .arg(Arg::new("X")
                .short('x')
                .long("do-not-compile")
                .action(ArgAction::SetTrue)
                .required(false)
                .default_value("false")
                .help("Does not execute 'clang' to compile llvm IR to executable")
                .requires("LL"))
            .arg(Arg::new("LL")
                .short('e')
                .long("emit-file")
                .value_name("file")
                .action(ArgAction::Set)
                .required(false)
                .value_parser(PathBufValueParser::new())
                .help("Sets filename for emitted llvm IR"))
            .arg(Arg::new("ONL")
                .short('n')
                .long("override-new-line-as-null")
                .action(ArgAction::SetTrue)
                .required(false)
                .default_value("false")
                .help("Makes '\\n'(0) be interpreted by Input command(',') as null(0)")
            )
        )
        .subcommand(command!("repl").about("Takes user input and compiles that as source")
            .arg(Arg::new("ON")
                .short('o')
                .long("output")
                .value_name("file")
                .required(false)
                .action(ArgAction::Set)
                .default_value({
                    #[cfg(windows)]
                    { "out.exe" }
                    #[cfg(not(windows))]
                    { "out" }
                })
                .help("Specifies output filename"))
            .arg(Arg::new("CC")
                .short('c')
                .long("cell-count")
                .value_name("count")
                .action(ArgAction::Set)
                .required(false)
                .default_value("30000")
                .value_parser(value_parser!(u16))
                .help("Specifies how many cells should there be"))
            .arg(Arg::new("X")
                .short('x')
                .long("do-not-compile")
                .action(ArgAction::SetTrue)
                .required(false)
                .default_value("false")
                .help("Does not execute 'clang' to compile llvm IR to executable")
                .requires("LL"))
            .arg(Arg::new("LL")
                .short('e')
                .long("emit-file")
                .value_name("file")
                .action(ArgAction::Set)
                .required(false)
                .value_parser(PathBufValueParser::new())
                .help("Sets filename for emitted llvm IR"))
            .arg(Arg::new("ONL")
                .short('n')
                .long("override-new-line-as-null")
                .action(ArgAction::SetTrue)
                .required(false)
                .default_value("false")
                .help("Makes '\\n'(0) be interpreted by Input command(',') as null(0)")
            )
        )
        .subcommand(command!("about").about("Prints about this software and of its licence"));

    let matches = cmd.clone().get_matches();


    match matches.subcommand() {
        Some(("about", _)) => {
            println!("
  Brainfck compiler
  Copyright (C) 2025  František Slivko <slivko.frantisek@gmail.com>

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <https://www.gnu.org/licenses/>.
        ");
            Err(false)
        }
        Some(("compile", cmd)) => {
            let name = cmd.get_one::<String>("FL").unwrap();
            if !std::fs::exists(&name).unwrap_or(false) {
                eprintln!("file '{name}' does not exist");
                return Err(true);
            }
            let source = PathBuf::from(name);

            let llvm_emit = match cmd.get_one::<PathBuf>("LL") {
                None => { None }
                Some(t) => {
                    let mut t = (*t).clone();
                    t.set_extension("ll");
                    Some(t)
                },
            };

            let run_clang = cmd.get_flag("X").not();

            let output = PathBuf::from(cmd.get_one::<String>("ON").expect("expected to not fail due to default value being set"));

            let cells_count = *cmd.get_one::<u16>("CC").unwrap();

            let override_new_line_to_null = cmd.get_flag("ONL");

            let code = std::fs::read_to_string(name).unwrap();

            Ok(Args::Compile{code, ca: CompileArgs { source }, sa: SettingsArgs {
                llvm_emit,
                run_clang,
                output,
                cells_count,
                override_new_line_to_null,
            }})
        }
        Some(("repl", cmd)) => {

            let llvm_emit = match cmd.get_one::<PathBuf>("LL") {
                None => { None }
                Some(t) => Some((*t).clone()),
            };

            let run_clang = cmd.get_flag("X").not();

            let output = PathBuf::from(cmd.get_one::<String>("ON").expect("expected to not fail due to default value being set"));

            let cells_count = *cmd.get_one::<u16>("CC").unwrap();

            let override_new_line_to_null = cmd.get_flag("ONL");

            println!("Welcome to REPL mode used to compile brainf*ck from user input and not from a file.");
            println!("Write 'exit' on empty line to proceed with compilation.");
            let mut ot = String::new();
            loop {
                let mut s = String::new();
                _ = std::io::stdin().read_line(&mut s);
                if s.starts_with("exit") {
                    break;
                } else {
                    ot.push_str(s.as_str());
                }
            }

            Ok(Args::Repl { code: ot, sa: SettingsArgs {
                llvm_emit,
                run_clang,
                output,
                cells_count,
                override_new_line_to_null,
            } })
        }
        _ => {
            _ = cmd.print_help();
            Err(true)
        }
    }
}

fn main() {
    match run() {
        Ok(_) => { std::process::exit(0) }
        Err(_) => { std::process::exit(1) }
    }
}

fn run() -> Result<(), ()> {

    // (Token, count)
    let mut tokens: Vec<(Token, usize, usize)> = vec![];

    let pa = match process_args() {
        Ok(v) => { v }
        Err(true) => { return Err(()); }
        Err(false) => { return Ok(()) }
    };

    let (input, source, sa) = match pa {
        Args::Compile { code, ca, sa} => { (code, match ca { CompileArgs { source} => Some(source)}, sa) }
        Args::Repl { code, sa} => { (code, None, sa) }
    };


    let (output_file, llvm_ir_filename, emit_llvm_ir, override_enter_to_null, cell_count, clang_additional_arguments, run_clang) = match sa {
        SettingsArgs { llvm_emit, run_clang, output, cells_count, override_new_line_to_null } => {
            let emit = llvm_emit.is_some();
            let ll = llvm_emit.unwrap_or(output.clone().with_extension(".ll"));
            (output, ll, emit, override_new_line_to_null, cells_count, vec!["-O3"], run_clang)
        }
    };

    let has_clang = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "clang --version"])
            .output().map(|s| s.status.success()).unwrap_or(false)
    } else {
        std::process::Command::new("clang")
            .arg("--version")
            .output().map(|s| s.status.success()).unwrap_or(false)
    };

    if !has_clang && run_clang {
        eprintln!("requires 'clang'");
        return Err(());
    }


    let mut ch_iter = input.chars().enumerate().peekable();

    while let Some((i, c)) = ch_iter.next() {
        let t = Token::from_char(&c);
        if t.is_none() { continue };
        let t = t.unwrap();
        let mut cnt: usize = 1;
        while ch_iter.peek().is_some_and(|(_, n)| *n == c) {
            cnt += 1;
            ch_iter.next();
        }
        tokens.push((t, cnt, i));
    }

    let mut f = String::new();
    write!(f, "source_filename = \"{}\"\n", source.map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "console".to_string()));
    write_init(&mut f, cell_count);
    
    let mut brcks: Vec<(usize, usize)> = vec![];
    let mut _consts: Vec<(String, String)> = vec![];

    for (i, (token, cnt, pos)) in tokens.into_iter().enumerate() {
        process_token(token, cnt, i, pos, &mut brcks, &mut _consts, override_enter_to_null, cell_count, &mut f);
    }

    write!(f, "; ------- END ------ ;\n");
    // write!(f, "  call void @putchar(i8 10)\n");
    write!(f, "  br label %exit\n");
    write!(f, "exit:\n");
    write!(f, "  %exit_v = load i8, ptr %exit_code\n");
    write!(f, "  ret i8 %exit_v\n");
    write!(f, "}}\n");
    for (c_name, c_val) in _consts {
        write!(f, "@{c_name} = private constant [{} x i8] c\"{}\\00\"\n",c_val.len()+1, c_val.escape_debug());
    }


    _ = std::fs::write(&llvm_ir_filename, f);

    if run_clang {
        print!("invoking clang...");
        let o = std::process::Command::new("clang").arg(&llvm_ir_filename).arg("-o").arg(&output_file).args(clang_additional_arguments).output().unwrap();
        print!("\r");
        if !o.status.success() {
            eprintln!("clang failed with:\n{}", String::from_utf8(o.stderr).unwrap());
            return Err(());
        } else {
            println!("compilation successful");
            println!("written executable '{}'", output_file.display());
            if emit_llvm_ir {
                println!("emitted LLVM IR successfully to '{}'", llvm_ir_filename.display());
            }
        }
    } else {
        println!("emitted LLVM IR successfully to '{}'", llvm_ir_filename.display());
    }

    if !emit_llvm_ir {
        _ = std::fs::remove_file(&llvm_ir_filename);
    }

    Ok(())
}

fn write_init(f: &mut String, cell_count: u16) {
    write!(f, "{}\n", PREABLE);
    write!(f, "
@bounds_r_panic_msg = private constant [{} x i8] c\"exceeded bounds check (larger than {cell_count})\\00\";
@bounds_l_panic_msg = private constant [39 x i8] c\"exceeded bounds check (smaller than 0)\\00\";

define i8 @code() {{
init:
  %exit_code = alloca i8
  store i8 0, ptr %exit_code

  %panic_msg = alloca i8*
  %panic_pos = alloca i16
  store i8* @none, ptr %panic_msg
  %arr = alloca [{cell_count} x i8]
  call void @llvm.memset.p0.i32(ptr %arr, i8 0, i32 {cell_count}, i1 0)
  %pos = alloca i16
  store i16 0, ptr %pos
  br label %code
panic:
  %msg = load ptr, i8* %panic_msg
  %p_pos = load i16, ptr %panic_pos
  store i8 1, ptr %exit_code
  call void @printf(ptr @panic_f, ptr %msg, i16 %p_pos)
  br label %exit
code:
", "exceeded bounds check (larger than )".len() + cell_count.to_string().len() + 1);
}

fn process_token(t: Token, count: usize, index: usize, file_pos: usize, brkcs: &mut Vec<(usize, usize)>, _consts: &mut Vec<(String, String)>, override_enter_to_null: bool, cell_count: u16, f: &mut String) {

    // comment start
    write!(f, "; ---- {index} | {:?} x {count} | at char {} ----\n", t, file_pos+1);
    match t {
        Token::Right => {
            write!(f, "  %pos{index} = load i16, ptr %pos\n");
            write!(f, "  %pos{index}n = add i16 %pos{index}, {count}\n");
            // bounds check
            write!(f, "  %rbound{index} = icmp uge i16 %pos{index}n, {cell_count}\n");
            write!(f, "  br i1 %rbound{index}, label %bounds_panic{index}, label %continue{index}\n");
            write!(f, "bounds_panic{index}:\n");
            write!(f, "  store i16 {}, ptr %panic_pos\n", file_pos+1);
            write!(f, "  store i8* @bounds_r_panic_msg, ptr %panic_msg\n");
            write!(f, "  br label %panic\n");
            write!(f, "continue{index}:\n");
            // end
            write!(f, "  store i16 %pos{index}n, ptr %pos\n");
        }
        Token::Left => {
            write!(f, "  %pos{index} = load i16, ptr %pos\n");
            write!(f, "  %pos{index}n = sub i16 %pos{index}, {count}\n");

            // bounds check
            write!(f, "  %lbound{index} = icmp ugt i16 %pos{index}n, %pos{index}");
            write!(f, "  br i1 %lbound{index}, label %bounds_panic{index}, label %continue{index}\n");
            write!(f, "bounds_panic{index}:\n");
            write!(f, "  store i16 {}, ptr %panic_pos\n", file_pos+1);
            write!(f, "  store i8* @bounds_l_panic_msg, ptr %panic_msg\n");
            write!(f, "  br label %panic\n");
            write!(f, "continue{index}:\n");
            // end

            write!(f, "  store i16 %pos{index}n, ptr %pos\n");
        }
        Token::Increment => {
            write!(f, "  %pos{index} = load i16, ptr %pos\n");
            write!(f, "  %t{index} = getelementptr i8, ptr %arr, i16 %pos{index}\n");
            write!(f, "  %c{index} = load i8, ptr %t{index}\n");
            write!(f, "  %c{index}n = add i8 %c{index}, {count}\n");
            write!(f, "  store i8 %c{index}n, ptr %t{index}\n");
        }
        Token::Decrement => {
            write!(f, "  %pos{index} = load i16, ptr %pos\n");
            write!(f, "  %t{index} = getelementptr i8, ptr %arr, i16 %pos{index}\n");
            write!(f, "  %c{index} = load i8, ptr %t{index}\n");
            write!(f, "  %c{index}n = sub i8 %c{index}, {count}\n");
            write!(f, "  store i8 %c{index}n, ptr %t{index}\n");
        }
        Token::Output => {
            write!(f, "  %pos{index} = load i16, ptr %pos\n");
            write!(f, "  %t{index} = getelementptr i8, ptr %arr, i16 %pos{index}\n");
            write!(f, "  %c{index} = load i8, ptr %t{index}\n");
            for _ in 0..count {
                write!(f, "  call void @putchar(i8 %c{index})\n");
            }
        }
        Token::Input => {
            for _ in 0..count {
                write!(f, "  %nc{index} = call i8 @getchar()\n");
                write!(f, "  %pos{index} = load i16, ptr %pos\n");
                write!(f, "  %t{index} = getelementptr i8, ptr %arr, i16 %pos{index}\n");

                if override_enter_to_null {
                    write!(f, "  %eof_is{index} = icmp eq i8 %nc{index}, 10\n");
                    write!(f, "  %nsc{index} = select i1 %eof_is{index}, i8 0, i8 %nc{index}\n");
                    write!(f, "  store i8 %nsc{index}, ptr %t{index}\n");
                } else {
                    write!(f, "  store i8 %nc{index}, ptr %t{index}\n");
                }

            }
        }
        Token::LoopStart => {
            for k in 0..count {
                write!(f, "  %pos{index}_{k} = load i16, ptr %pos\n");
                write!(f, "  %t{index}_{k} = getelementptr i8, ptr %arr, i16 %pos{index}_{k}\n");
                write!(f, "  %c{index}_{k} = load i8, ptr %t{index}_{k}\n");
                write!(f, "  %cmp_rs{index}_{k} = icmp eq i8 %c{index}_{k}, 0\n");
                write!(f, "  br i1 %cmp_rs{index}_{k}, label %skip{index}_{k}, label %loop{index}_{k}\n");
                write!(f, "loop{index}_{k}:\n");
                brkcs.push((index, k));
            }
        }
        Token::LoopEnd => {
            if brkcs.is_empty() { panic!("no matching loop bracket"); }
            for k in 0..count {
                let (n1, n2) = brkcs.pop().unwrap();
                write!(f, "  %pos{index}_{k} = load i16, ptr %pos\n");
                write!(f, "  %t{index}_{k} = getelementptr i8, ptr %arr, i16 %pos{index}_{k}\n");
                write!(f, "  %c{index}_{k} = load i8, ptr %t{index}_{k}\n");
                write!(f, "  %cmp_rs{index}_{k} = icmp ne i8 %c{index}_{k}, 0\n");
                write!(f, "  br i1 %cmp_rs{index}_{k}, label %loop{n1}_{n2}, label %skip{n1}_{n2}\n");
                write!(f, "skip{n1}_{n2}:\n");
            }

        }
        Token::PrintNumber => {
            write!(f, "  %pos{index} = load i16, ptr %pos\n");
            write!(f, "  %t{index} = getelementptr i8, ptr %arr, i16 %pos{index}\n");
            write!(f, "  %c{index} = load i8, ptr %t{index}\n");
            for _ in 0..count {
                write!(f, "  call void @printf(ptr @i_print, i8 %c{index})\n");
            }
        }
    }

    // end comment
    write!(f, "; ------------\n\n");
}

