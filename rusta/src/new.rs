use std::fs::{canonicalize, create_dir};
use std::path::Path;

use exitfailure::ExitFailure;
use failure::ResultExt;

pub fn new_site (name: &str) -> Result<(), ExitFailure> {
    let path = Path::new(name);

    create_dir(path)?;

    let config =
        "baseURL = \"http://example.org/\" \
        languageCode = \"en-us\" \
        title = \"My New Rusta Site\"";

    create_dir(path.join("archetypes"))?;
    create_dir(path.join("content"))?;
    create_dir(path.join("data"))?;
    create_dir(path.join("layouts"))?;
    create_dir(path.join("static"))?;
    create_dir(path.join("themes"))?;
    create_file(&path.join("config.toml"), &config)?;

    println!("Congratulations! Your new Rusta site is created in {:?}", canonicalize(path).unwrap());

    Ok(())
}
