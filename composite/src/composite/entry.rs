pub trait Entry {
    fn get_size(&self) -> u32;
    fn print_list(&self, prefix: String);
}
