#![allow(clippy::result_large_err)]

use clap::Parser as ClapParser;
use havo::{
    cli::Cli,
    err::MsgWithPos,
    gccjit::Codegen as GccJITCodegen,
    // optimize::const_eval,
    semantic::*,
    syntax::{ast::*, lexer::reader::Reader, parser::Parser},
    Context,
};

use havo::cli::Backend;
use std::path::PathBuf;

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
