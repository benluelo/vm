// #![feature(deref_patterns)]
#![warn(clippy::panic, clippy::unwrap_in_result)]
use std::{
    fs,
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::bail;
use argh::{FromArgValue, FromArgs};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{Parser, error::Rich};
use const_hex::ToHexExt;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vm::{
    Vm,
    assembler::parse_asm,
    mir::{
        self, CheckCtx, Ctx,
        parse::print_ast,
        pass::{ConstEval, ConstProp, DeadCodeRemoval, DefInline, LoopUnroll, Pass},
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

    /// if this flag is provided, the source file will be treated as a assembly
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

    /// if this flag is provided, the source file will be treated as a assembly
    /// file rather than a code file.
    #[argh(switch)]
    pub asm: bool,

    /// if this flag is provided, the source file will be treated as an object
    /// file rather than a code file.
    #[argh(switch)]
    pub obj: bool,

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
                        let mut ctx = CheckCtx::new("root");
                        ctx.check(&obj)?;
                        let obj = ConstEval::new().run(&ctx, obj);

                        let mut ctx = CheckCtx::new("root");
                        ctx.check(&obj)?;
                        let obj = DefInline::new().run(&ctx, obj);

                        let mut ctx = CheckCtx::new("root");
                        ctx.check(&obj)?;
                        let obj = DefInline::new().run(&ctx, obj);

                        let mut ctx = CheckCtx::new("root");
                        let obj = ctx.check_with(&obj, &mut LoopUnroll)?;

                        println!("{}", print_ast(&obj));

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
            obj,
            input,
            input_file,
            input_hex,
        }) => {
            if obj && asm {
                bail!("--asm is incompatible with --obj")
            }

            let obj = if obj {
                fs::read(&file)?
            } else if asm {
                let file = fs::read_to_string(&file)?;
                match parse_asm().parse(&file).into_result() {
                    Ok(obj) => obj.assemble(),
                    Err(errs) => {
                        report_errors(&fs::read_to_string(&file)?, errs);
                        return Ok(());
                    }
                }
            } else {
                let file = fs::read_to_string(&file)?;
                match mir::parse::grammar().block.parse(&file).into_result() {
                    Ok(ast) => {
                        let mut ctx = CheckCtx::new("root");
                        ctx.check(&ast)?;
                        let ast = DefInline::new().run(&ctx, ast);

                        let mut ctx = CheckCtx::new("root");
                        ctx.check(&ast)?;
                        let ast = DefInline::new().run(&ctx, ast);

                        let mut ast = ast;

                        for i in 1..=3 {
                            let mut ctx = CheckCtx::new("root");
                            ast = ctx.check_with(&ast, &mut LoopUnroll)?;

                            for i in 1.. {
                                let mut ctx = CheckCtx::new("root");
                                let new_ast = ctx.check_with(&ast, &mut ConstProp)?;

                                let mut ctx = CheckCtx::new("root");
                                ctx.check(&new_ast)?;
                                let new_ast = DefInline::new().run(&ctx, new_ast);

                                let mut ctx = CheckCtx::new("root");
                                ctx.check(&new_ast)?;
                                let new_ast = ConstEval::new().run(&ctx, new_ast);

                                let mut ctx = CheckCtx::new("root");
                                let new_ast = ctx.check_with(&new_ast, &mut DeadCodeRemoval)?;

                                if new_ast == ast {
                                    info!("ran const prop/eval loop {i} times");
                                    break;
                                } else {
                                    ast = new_ast;
                                }
                            }
                        }

                        #[expect(
                            clippy::unwrap_in_result,
                            reason = "if this errors we probably have bigger issues to deal with"
                        )]
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("???")
                            .as_nanos()
                            - 1784300000000000000;
                        fs::write(format!("out-{now}.mir"), print_ast(&ast))?;
                        // fs::write("after.mir", print_ast(&ast2))?;

                        // println!("{}", print_ast(&ast));
                        let mut ctx = Ctx::new_root();
                        ctx.compile(&ast)?;
                        ctx.into_object().assemble()
                    }
                    Err(errs) => {
                        report_errors(&fs::read_to_string(&file)?, errs);
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
            let now = Instant::now();
            let res = vm.run();
            let elapsed = now.elapsed();
            match res {
                Ok(res) => {
                    println!("time: {}", elapsed.as_secs_f64());
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
                Err(err) => {
                    println!("err: {err}");
                    println!("cycles: {}", vm.cycles);
                    // println!("{}", const_hex::encode(vm.memory));
                }
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
