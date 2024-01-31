use gg::{get_cmd, get_matches, run};


fn main() {
    let cmd = get_cmd();
    let matches = get_matches(&cmd);

    run(cmd, matches);
}
