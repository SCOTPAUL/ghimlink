extern crate git2;
extern crate regex;
#[macro_use] extern crate text_io;
#[macro_use] extern crate clap;

use git2::{Repository};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::io;
use std::io::Write;

struct ImageDetails {
    alt_text: String,
    image_path: PathBuf
}

fn main() {
    let matches = app_from_crate!()
                    .args_from_usage("-b --branch=[BRANCH_NAME] 'Sets the file's branch name (defaults to master)'
                                      -a --alt_text=[ALT_TEXT] 'Sets the alt-text for the image'
                                     <IMAGE_PATH> 'Path to the image file'")
                    .get_matches();

    let image_path = matches.value_of("IMAGE_PATH").unwrap();
    let branch_name = matches.value_of("branch").unwrap_or("master");
    let alt_text = matches.value_of("alt_text");

    let details = get_image_link_details(&image_path, alt_text);

    match details {
        Some(details) => {
            match get_remote_url() {
                Ok(remote_name) => println!("![{}]({}{}/{}/{})",
                    details.alt_text,
                    remote_name,
                    "raw",
                    branch_name,
                    details.image_path.to_str().unwrap()),
                Err(e) => println!("{:?}", e)
            };
        },
        None => eprintln!("Error: Invalid image details")
    };

}

fn get_image_link_details(image_path: &str, alt_text_option: Option<&str>) -> Option<ImageDetails> {
    let path = get_relative_image_path(image_path)?;

    let alt_text = match alt_text_option {
        Some(alt_text) => alt_text.to_string(),
        None => {
            print!("Enter alt text: ");
            io::stdout().flush().unwrap();
            let alt_text: String = read!();
            alt_text
        }
    };

    Some(ImageDetails { alt_text: alt_text, image_path: path })
}

fn get_relative_image_path(image_path: &str) -> Option<PathBuf> {
    let full_path = match Path::new(image_path).canonicalize() {
        Ok(path) => Some(path),
        Err(_e) => None
    }?;

    let repo = match Repository::open_from_env() {
        Ok(repo) => Some(repo),
        Err(_e) => None
    }?;

    let repo_path = repo.path().parent()?;
    let relative_path = full_path.strip_prefix(repo_path).unwrap().to_path_buf();

    Some(relative_path)

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
