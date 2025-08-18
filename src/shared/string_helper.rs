pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
}
impl ToWide for str {
    fn to_wide(&self) -> Vec<u16> {
        self.encode_utf16().chain(std::iter::once(0)).collect()
    }
}