pub trait DescribableError {
    fn message(&self) -> String;
    fn kind(&self) -> String;
    fn print(&self) {
        eprintln!(
            "{}: {}",
            self.kind(),
            self.message(),
        );
    }
}
