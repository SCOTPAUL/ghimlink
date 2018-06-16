extern crate git2;
use git2::{Repository};



fn main() {
    println!("Hello, world!");

    match get_remote_name() {
        Ok(name) => println!("{}", name),
        Err(e) => println!("{:?}", e)
    }
}

fn get_remote_name() -> Result<String, git2::Error> {
    let repo = Repository::open_from_env()?;

    let remote = repo.find_remote("origin")?;

    let name = remote.url().unwrap();

    Ok(name.to_string())

}