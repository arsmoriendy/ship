macro_rules! docker {
    ($($args:expr),+) => {
        Command::new("docker")
            .args([$($args),+])
    };
}
