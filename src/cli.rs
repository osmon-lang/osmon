use clap::{Parser, ValueEnum};
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

#[derive(Debug, Parser)]
#[command(name = "osmon")]
#[command(about = "The Osmon Compiler", long_about = None)]
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
