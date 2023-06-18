use std::env;
use std::fs;
use std::num::Wrapping;
use std::process;
use tcpuemu::Cpu;

fn get_filename_from_args(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();
    match args.next() {
        Some(filename) => Ok(filename),
        None => Err("Usage: tcpuemu filename"),
    }
}

fn main() {
    let filename = get_filename_from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mem_init = match fs::read(&filename) {
        Ok(result) => result,
        Err(message) => panic!("{message}: {filename}"),
    };
    let mem_init = mem_init.into_iter().map(Wrapping).collect();

    let mut cpu = Cpu::new(Some(mem_init));
    cpu.run();
}
