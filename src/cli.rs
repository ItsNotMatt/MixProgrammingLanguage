use std::env;

#[derive(PartialEq, PartialOrd)]
pub enum CLI {
    Tokenize,
    Run,
}

pub fn get_args() -> CLI {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "-t" {
        return CLI::Tokenize;
    }
    return CLI::Run
}
