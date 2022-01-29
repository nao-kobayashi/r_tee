pub trait TeeWriter {
    fn write(&mut self, line: &str);
}