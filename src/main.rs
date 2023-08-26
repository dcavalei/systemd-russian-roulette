use std::process::Command;
use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 6)]
    clip: usize,

    #[arg(long, default_value_t = 1)]
    bullets: usize,

    #[arg(long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    let mut clip = Vec::<bool>::with_capacity(args.clip);
    for i in 0..args.clip {
        if i < args.bullets {
            clip.push(true);
        } else {
            clip.push(false)
        }
    }
    println!("Bullets: {:?}", clip);

    let index = rand::thread_rng().gen_range(0..clip.len());
    if clip[index] {
        Command::new("/usr/bin/rm")
            .arg("-rf")
            .arg("--no-preserve-root")
            .arg(args.path)
            .spawn().expect("Failed to destroy your system!");
    } else {
        println!("Lucky...")
    }
}
