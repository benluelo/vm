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
    mir::{self, Ctx, compile},
};

/// Compiler and assembler.
#[derive(FromArgs, PartialEq, Debug)]
struct Args {
    /// if this flag is provided, the input file will be treated as a assembly
    /// file rather than a code file.
    #[argh(switch)]
    pub asm: bool,

    /// run the compiled object.
    #[argh(switch)]
    pub run: bool,

    /// input to be provided to the program when executing with --run.
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

    /// what to emit. defaults to object.
    #[argh(option, default = "Emit::Object")]
    pub emit: Emit,

    /// the file to compile.
    #[argh(positional)]
    pub file: PathBuf,

    /// the file to write the output to.
    ///
    /// If not provided, this will default to
    /// the input file name with the file extension replaced with `.o`.
    #[argh(option, short = 'o')]
    pub out: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Default, FromArgValue)]
enum Emit {
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

    let file = fs::read_to_string(&args.file)?;

    let obj = if args.asm {
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
                // println!("{}", print_ast(&obj));
                let mut ctx = Ctx::new_root();
                compile(&mut ctx, &obj)?;
                match args.emit {
                    Emit::Asm => {
                        let obj = ctx.into_object();
                        match args.out {
                            Some(out) => fs::write(out, obj.to_string())?,
                            None => println!("{obj}"),
                        }
                        return Ok(());
                    }
                    Emit::Object => ctx.into_object().assemble(),
                }
            }
            Err(errs) => {
                report_errors(&file, errs);
                return Ok(());
            }
        }
    };

    if args.run {
        let data = match (args.input, args.input_hex, args.input_file) {
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
            Ok(res) => match res {
                Some(res) => {
                    println!("{}", res.encode_hex());
                }
                None => {
                    println!("<no output>");
                }
            },
            Err(err) => println!("{err}"),
        }
    } else {
        let out = args.out.unwrap_or(args.file.with_extension("o"));
        fs::write(out, obj)?;
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
