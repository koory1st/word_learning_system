pub mod util;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
#[macro_use]
extern crate lazy_static;
fn main() {
    // read unlearned file to set
    let unlearned_set: HashSet<String> = util::read_file_2_set("./resources/unlearned");
    // read learned file to set
    let learned_set: HashSet<String> = util::read_file_2_set("./resources/learned");

    // get unlearned words from unlearned set which are not in learned set
    let difference: HashSet<&String> = unlearned_set.difference(&learned_set).collect();

    // write unlearned words to unlearned file
    util::write_set_2_file("./resources/output", difference);
}
