use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use structopt::StructOpt;
use tiktoken_rs::p50k_base;

#[derive(StructOpt, Debug)]
#[structopt(name = "token-counter", about = "Count tokens in files.")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<std::path::PathBuf>,
}

fn token_count(content: &str) -> usize {
    let bpe = p50k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(content);
    tokens.len()
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let mut total = 0;
    let mut file_count = 0;

    if opt.files.is_empty() {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        let count = token_count(&content);
        println!("{}", count);
    } else {
        for filename in &opt.files {
            let path = Path::new(&filename);
            let file = match File::open(&path) {
                Err(why) => {
                    eprintln!("Error: {}", why);
                    continue;
                }
                Ok(file) => file,
            };

            let mut content = String::new();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut content)?;

            let count = token_count(&content);
            println!("{:8} {}", count, filename.display());
            total += count;
            file_count += 1;
        }

        if file_count > 1 {
            println!("{:8} total", total);
        }
    }

    Ok(())
}
