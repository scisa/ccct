pub mod cli;
pub mod util;

use crate::cli::arguments::Args;


fn main() {
    let args = Args::get_args();
    println!("{:?}", args);
}
