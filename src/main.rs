#![warn(clippy::panic, clippy::unwrap_in_result)]
use std::{fs, path::PathBuf};

use anyhow::bail;
use argh::{FromArgValue, FromArgs};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{Parser, error::Rich};
use const_hex::ToHexExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vm::{
    Vm,
    assembler::parse_asm,
    mir::{
        self, CheckCtx, Ctx,
        parse::print_ast,
        pass::{ConstEval, DefInline, Pass},
    },
};

/// Compiler and assembler.
#[derive(FromArgs, PartialEq, Debug)]
struct Args {
    #[argh(subcommand)]
    cmd: Cmd,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Cmd {
    Check(CheckCmd),
    Build(BuildCmd),
    Run(RunCmd),
    // Assemble {},
}

/// check a .mir file.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "check")]
pub struct CheckCmd {
    /// the file to check.
    #[argh(positional)]
    pub file: PathBuf,
}

/// build a .mir file.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
pub struct BuildCmd {
    /// the file to compile.
    #[argh(positional)]
    pub file: PathBuf,

    /// if this flag is provided, the srouce file will be treated as a assembly
    /// file rather than a code file.
    #[argh(switch)]
    pub asm: bool,

    /// the file to write the output to.
    ///
    /// If not provided, this will default to
    /// the input file name with the file extension replaced with `.o`.
    #[argh(option, short = 'o')]
    pub out: Option<PathBuf>,

    /// what to emit. defaults to object.
    #[argh(option, default = "Emit::Object")]
    pub emit: Emit,
}

/// run either a .mir or .asm file.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
pub struct RunCmd {
    /// the file to compile.
    #[argh(positional)]
    pub file: PathBuf,

    /// if this flag is provided, the srouce file will be treated as a assembly
    /// file rather than a code file.
    #[argh(switch)]
    pub asm: bool,

    /// input to be provided to the program.
    ///
    /// Incompatible with --input-file.
    #[argh(option)]
    pub input: Option<String>,

    /// path to the input to be provided to the program when executing with
    /// --run.
    ///
    /// Incompatible with --input.
    #[argh(option)]
    pub input_file: Option<PathBuf>,

    /// whether to treat --input as hex.
    #[argh(switch)]
    pub input_hex: bool,
}

#[derive(Debug, Clone, PartialEq, Default, FromArgValue)]
pub enum Emit {
    Asm,
    #[default]
    Object,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args = argh::from_env::<Args>();

    match args.cmd {
        Cmd::Check(CheckCmd { file }) => {
            let source = fs::read_to_string(&file)?;

            match mir::parse::grammar().block.parse(&source).into_result() {
                Ok(obj) => {
                    let mut ctx = CheckCtx::new("root");
                    ctx.check(&obj)?;
                }
                Err(errs) => {
                    report_errors(&source, errs);
                    return Ok(());
                }
            }
        }
        Cmd::Build(BuildCmd {
            file,
            asm,
            out,
            emit,
        }) => {
            let source = fs::read_to_string(&file)?;

            let obj = if asm {
                match parse_asm().parse(&source).into_result() {
                    Ok(obj) => obj.assemble(),
                    Err(errs) => {
                        report_errors(&source, errs);
                        return Ok(());
                    }
                }
            } else {
                match mir::parse::grammar().block.parse(&source).into_result() {
                    Ok(obj) => {
                        // println!("{}", print_ast(&obj));
                        let mut ctx = Ctx::new_root();
                        ctx.compile(&obj)?;
                        match emit {
                            Emit::Asm => {
                                let obj = ctx.into_object();
                                match out {
                                    Some(out) => fs::write(out, obj.to_string())?,
                                    None => println!("{obj}"),
                                }
                                return Ok(());
                            }
                            Emit::Object => ctx.into_object().assemble(),
                        }
                    }
                    Err(errs) => {
                        report_errors(&source, errs);
                        return Ok(());
                    }
                }
            };

            let out = out.unwrap_or(file.with_extension("o"));
            fs::write(out, obj)?;
        }
        Cmd::Run(RunCmd {
            file,
            asm,
            input,
            input_file,
            input_hex,
        }) => {
            let file = fs::read_to_string(&file)?;

            let obj = if asm {
                match parse_asm().parse(&file).into_result() {
                    Ok(obj) => obj.assemble(),
                    Err(errs) => {
                        report_errors(&file, errs);
                        return Ok(());
                    }
                }
            } else {
                match mir::parse::grammar().block.parse(&file).into_result() {
                    Ok(obj) => {
                        let mut ctx = CheckCtx::new("root");
                        ctx.check(&obj)?;
                        let obj = ConstEval::new().run(&ctx, obj);

                        let mut ctx = CheckCtx::new("root");
                        ctx.check(&obj)?;
                        let obj = DefInline::new().run(&ctx, obj);

                        let mut ctx = CheckCtx::new("root");
                        ctx.check(&obj)?;
                        let obj = DefInline::new().run(&ctx, obj);

                        println!("{}", print_ast(&obj));
                        let mut ctx = Ctx::new_root();
                        ctx.compile(&obj)?;
                        ctx.into_object().assemble()
                    }
                    Err(errs) => {
                        report_errors(&file, errs);
                        return Ok(());
                    }
                }
            };

            let data = match (input, input_hex, input_file) {
                (None, true, None) => bail!("--input-hex requires --input"),
                (None, false, None) => vec![],
                (None, true, Some(path)) => const_hex::decode(fs::read(path)?)?,
                (None, false, Some(path)) => fs::read(path)?,
                (Some(input), true, None) => const_hex::decode(input)?,
                (Some(input), false, None) => input.into_bytes(),
                (Some(_), _, Some(_)) => {
                    bail!("--input is mutually exclusive with --input-file")
                }
            };

            let mut vm = Vm::new(obj, data);
            let res = vm.run();
            match res {
                Ok(res) => {
                    println!("total cycles: {}", vm.cycles);
                    println!("binary size: {}", vm.code.len());
                    match res {
                        Some(res) => {
                            println!("output: {}", res.encode_hex());
                        }
                        None => {
                            println!("output: <no output>");
                        }
                    }
                }
                Err(err) => println!("{err}"),
            }
        }
    }

    Ok(())
}

fn report_errors(file: &str, errs: Vec<Rich<'_, char>>) {
    for e in errs {
        Report::build(ReportKind::Error, ((), e.span().into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(e.to_string())
            .with_label(
                Label::new(((), e.span().into_range()))
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .print(Source::from(&file))
            .unwrap()
    }
}
