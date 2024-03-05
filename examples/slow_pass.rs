fn main() {
    let mut line = String::new();

    while matches!(std::io::stdin().read_line(&mut line), Ok(n) if n > 0) {
        print!("{}", line);
        line.clear();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
