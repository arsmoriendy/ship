macro_rules! docker {
    ($($args:expr),+) => {
        Command::new("docker")
            .args([$($args),+])
            .output()
            .with_context(|| "Failed spawning docker, make sure docker is installed")?
    };
}
