pub trait DisplayImpl {
    fn raw_open(&self);
    fn raw_print(&mut self);
    fn raw_close(&self);
}
