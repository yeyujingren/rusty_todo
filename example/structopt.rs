use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
  #[structopt(short, long)]
  debug: bool,
}

fn main() {
  let opt = Opt::from_args();
  printlln!("{:#?}", opt);
}

