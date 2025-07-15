use minigrep::Config;
use std::env;

// Esse programa replica o comportamento base do comando grep do Unix
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Erro ao tentar converter argumentos: {err}");
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Erro de aplicação: {}", e);
        std::process::exit(1);
    }
}
