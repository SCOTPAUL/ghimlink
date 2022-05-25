extern crate git2;
extern crate regex;
#[macro_use] extern crate text_io;
#[macro_use] extern crate clap;

use git2::{Repository};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::io;
use std::io::Write;
use std::error::Error;

#[derive(Debug)]
struct ImageDetails {
    alt_text: String,
    image_path: PathBuf
}

fn main() {
    let matches = App::new("folder-locker")
    .version("1.0")
    .author("Paul Cowie <paul.cowie@ntlworld.com>")
    .about("Does awesome things")
    .arg("-l, --lock 'Puts the tool in locking mode'")
    .arg("-u, --lock 'Puts the tool in unlocking mode'")
    .arg("<INPUT_FOLDER> 'Folder to lock or unlock'")
    .get_matches();

}

fn get_current_branch() -> Result<String, git2::Error> {
    let repo = Repository::open_from_env()?;
    let branch = repo.head()?;

    Ok(branch.shorthand().unwrap().to_string())
}

fn get_image_link_details(image_path: &str, alt_text_option: Option<&str>) -> Result<ImageDetails, Box<dyn Error>> {
    let path = get_relative_image_path(image_path)?;

    let alt_text = match alt_text_option {
        Some(alt_text) => alt_text.to_string(),
        None => {
            print!("Enter alt text: ");
            io::stdout().flush().unwrap();
            let alt_text: String = read!("{}\n");
            alt_text
        }
    };

    Ok(ImageDetails { alt_text, image_path: path })
}

fn get_relative_image_path(image_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let full_path = Path::new(image_path).canonicalize()?;

    let repo = Repository::open_from_env()?;

    let repo_path = repo.path().parent().ok_or("Repo path not found")?.canonicalize()?;

    let relative_path = full_path.strip_prefix(repo_path).unwrap().to_path_buf();

    Ok(relative_path)
}

fn get_remote_url() -> Result<String, git2::Error> {
    let repo = Repository::open_from_env()?;
    let remote = repo.find_remote("origin")?;
    let name = remote.url().unwrap().to_string();

    let https_name = if name.starts_with("git@") {
        let re = Regex::new(r"git@github\.com:(?P<username>.+)/(?P<repo_name>[^\.]+)(?:\.git)?").unwrap();
        let matches = re.captures(&name).unwrap();

        format!("https://github.com/{}/{}/", &matches["username"], &matches["repo_name"])
    }
    else {
        name
    };

    Ok(https_name)
}
