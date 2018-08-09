use clap::{App, Arg};
use manager::Manager;

fn build_cli<'a>(project_list: &[&'a str]) -> App<'a, 'a> {
    App::new("Goto project")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Easy and fast project switching in your shell!")
        .after_help(
            "Before usage write configuration of your projects list in ~/.goto-project.yaml",
        )
        .arg(
            Arg::with_name("project")
                .takes_value(true)
                .possible_values(project_list)
                .help("Open choosen project if passed, otherwise list them all."),
        )
}

pub fn run_cli() -> () {
    let manager = Manager::new(".goto-project.yaml");

    let project_list = manager.list_projects();
    let project_list: Vec<&str> = project_list.iter().map(|p| p.as_ref()).collect();

    let matches = build_cli(project_list.as_slice()).get_matches();

    match matches.value_of("project") {
        Some(project) => manager.open_project(project),
        None => {
            for project_name in project_list {
                println!("{}", project_name);
            }
        }
    }
}