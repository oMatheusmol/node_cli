mod ts;

use clap::{App, Arg};
use ts::create_project::create_project;

fn main() {
    let matches = App::new("CLI")
        .version("0.1.0")
        .author("Matheus Mol <matheusmol@hotmail.com>")
        .about("Cria um projeto com a linguagem e dependências escolhidas")
        .arg(
            Arg::with_name("language")
                .required(true)
                .takes_value(true)
                .help("Linguagem do projeto")
                .help("ts, js, py"),
        )
        .arg(
            Arg::with_name("action")
                .required(true)
                .takes_value(true)
                .help("new, add, remove"),
        )
        .arg(
            Arg::with_name("component")
                .required(true)
                .takes_value(true)
                .help("project, service, controller, usecase, repository, model"),
        )
        .arg(
            Arg::with_name("name")
                .required(true)
                .takes_value(true)
                .help("Nome do projeto"),
        )
        .get_matches();

    create_project(
        matches.value_of("name").unwrap(),
        matches.value_of("language").unwrap(),
    );
}
