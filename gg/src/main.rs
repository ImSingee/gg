use gg::{get_cmd, get_matches, run};


fn main() {
    let mut cmd = get_cmd();
    let matches = get_matches(&mut cmd);

    run(cmd, matches);
}
