# Ghimlink (Github Image Linker)

A simple command-line application for generating a Github-Flavoured-Markdown image link for a file in your repository.

## Simple Usage

For example, if we want to link to `gimli.png` and `link.jpg` in this repository's `res/` directory, we could do so as follows:

```bash
ghimlink ./res/gimli.png
Enter alt text: Gimli
![Gimli](https://github.com/SCOTPAUL/ghimlink/raw/master/res/gimli.png)

ghimlink ./res/link.jpg 
Enter alt text: Link
![Link](https://github.com/SCOTPAUL/ghimlink/raw/master/res/link.jpg)
```

Giving us:

![Gimli](https://github.com/SCOTPAUL/ghimlink/raw/master/res/gimli.png)
![Link](https://github.com/SCOTPAUL/ghimlink/raw/master/res/link.jpg)

Ghimlink will handle retreiving your Github remote data automatically (as long as you have a valid `origin` remote on Github).

## Advanced Usage

```
ghimlink 0.1.0
Paul Cowie <paul.cowie@ntlworld.com>
Generates a Github-Flavoured-Markdown formatted image link to an image in the same repository.

USAGE:
    ghimlink [OPTIONS] <IMAGE_PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --alt_text <ALT_TEXT>     Sets the alt-text for the image
    -b, --branch <BRANCH_NAME>    Sets the file's branch name (defaults to master)

ARGS:
    <IMAGE_PATH>    Path to the image file
```

## Installing

With Cargo:

```bash
cargo install ghimlink
```

## TODO

- Add support for remotes other than `origin`.
