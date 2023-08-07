#![allow(clippy::result_large_err)]

use clap::{Parser as ClapParser, ValueEnum};
use havo::{
    err::MsgWithPos,
    gccjit::Codegen as GccJITCodegen,
    // optimize::const_eval,
    semantic::*,
    syntax::{ast::*, lexer::reader::Reader, parser::Parser},
    Context,
};
use std::fmt::Display;
use std::path::PathBuf;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Backend {
    #[value(help = "Default backend, allows JIT and AOT compilation")]
    GccJIT,
    #[value(help = "C++ backend,still W.I.P")]
    CPP,
}

impl Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backend::GccJIT => write!(f, "gcc-jit"),
            Backend::CPP => write!(f, "cpp"),
        }
    }
}

#[derive(Debug, ClapParser)]
#[command(name = "havo")]
#[command(about = "The Havo Compiler", long_about = None)]
pub struct Cli {
    #[arg(required = true)]
    pub file: PathBuf,

    #[arg(
        short = 'O',
        long = "opt-level",
        default_value = "2",
        help = "Set optimization level"
    )]
    pub opt_level: u8,

    #[arg(long = "jit", help = "Use JIT compilation instead of AOT compilation")]
    pub jit: bool,

    #[arg(long = "emit-obj", help = "Output object file")]
    pub emit_obj: bool,

    #[arg(long = "emit-asm", help = "cPrint assembly to stdout")]
    pub emit_asm: bool,

    #[arg(short = 'o', long = "output", help = "Set output filename")]
    pub output: Option<PathBuf>,

    #[arg(long = "shared", help = "Output shared library (.dll or .so)")]
    pub shared: bool,

    #[arg(
        long = "emit-gimple",
        help = "Dump GIMPLE to stdout if gccjit backend used"
    )]
    pub emit_gimple: bool,

    #[arg(
        long = "backend",
        default_value_t = Backend::GccJIT,
        help = "Select backend",
    )]
    pub backend: Backend,

    #[arg(short = 'l', long = "link")]
    pub libraries_link: Vec<String>,

    #[arg(short = 'f')]
    pub gcc_opts: Vec<String>,

    #[arg(
        long = "consteval",
        help = "Enables constant folding and const function evaluating"
    )]
    pub const_eval: bool,

    #[arg(long = "print-ast", help = "Print program")]
    pub print_ast: bool,

    #[arg(
        long = "aggressive-eval",
        help = "try to evaluate normal (not constexpr) functions too"
    )]
    pub aggressive_eval: bool,
}

fn main() -> Result<(), MsgWithPos> {
    let cli: Cli = Cli::parse();

    let mut file = File {
        root: cli
            .file
            .parent()
            .unwrap_or(std::path::Path::new(""))
            .to_str()
            .unwrap()
            .to_owned(),
        src: String::new(),
        path: cli.file.to_str().unwrap().to_owned(),
        elems: vec![],
    };

    let reader = Reader::from_file(cli.file.to_str().unwrap()).unwrap();

    let mut parser = Parser::new(reader, &mut file);

    let err = parser.parse();
    if err.is_err() {
        println!("{}", err.clone().err().unwrap());
        std::process::exit(-1);
    }

    let mut ctx = Context::new(file);
    ctx.shared = cli.shared;
    ctx.emit_asm = cli.emit_asm;
    ctx.emit_obj = cli.emit_obj;
    ctx.jit = cli.jit;
    ctx.output = cli
        .output
        .map_or(String::new(), |e: PathBuf| e.to_str().unwrap().to_owned());
    ctx.opt = cli.opt_level;
    ctx.gimple = cli.emit_gimple;
    ctx.file.elems.extend(
        cli.libraries_link
            .iter()
            .map(|name| Elem::Link(havo::intern(name))),
    );
    let mut semantic = SemCheck::new(&mut ctx);

    semantic.run();

    use havo::eval::EvalCtx;
    let mut eval = EvalCtx::new(&mut ctx);
    eval.run();

    if cli.print_ast {
        for elem in ctx.file.elems.iter() {
            println!("{}", elem);
        }
    }

    match cli.backend {
        Backend::CPP => {
            use havo::ast2cpp::Translator;
            let mut translator = Translator::new(ctx);
            translator.run();
        }
        Backend::GccJIT => {
            let mut cgen = GccJITCodegen::new(&mut ctx, "HavoModule");
            for opt in cli.gcc_opts.iter() {
                cgen.ctx.add_command_line_option(opt);
            }
            cgen.compile();
        }
    }

    Ok(())
}
