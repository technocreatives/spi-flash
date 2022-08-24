use core::fmt;

pub struct HexSlice<T>(pub T)
where
    T: AsRef<[u8]>;

impl<T: AsRef<[u8]>> fmt::Debug for HexSlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        for (i, byte) in self.0.as_ref().iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            write!(f, "{:02x}", byte)?;
        }
        f.write_str("]")
    }
}

impl<T: AsRef<[u8]>> defmt::Format for HexSlice<T> {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::write!(f, "{=[u8]:x}", self.0.as_ref());
    }
}
