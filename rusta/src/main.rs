#[macro_use]
extern crate clap;
use clap::App;

use exitfailure::ExitFailure;
use failure::ResultExt;

mod build;
mod new;
mod serve;

fn main() -> Result<(), ExitFailure> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("new", Some(matches)) => {
            if matches.is_present("name") {
                new::new_site(matches.value_of("name").unwrap());
            }
        }
        ("build", Some(_matches)) => {
            build::build_site();
        }
        ("serve", Some(_matches)) => {
            serve::serve_site();
        }
        _ => unreachable!()
    }

    Ok(())
}
