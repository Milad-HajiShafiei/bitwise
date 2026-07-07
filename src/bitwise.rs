use std::io::Error;

/// A wrapper around a `u64` that provides bit-level operations,
/// treating the value as a sequence of 64 individually addressable bits
/// (indices `0..=63`, where index `0` is the least-significant bit).
///
/// Useful for bitboards, flag sets, or any scenario where individual bits
/// of a 64-bit integer need to be read, set, cleared, or flipped safely
/// (i.e., with bounds checking on the bit index).
pub struct Bitwise {
    /// The underlying 64-bit value being manipulated.
    pub data: u64,
}

impl Bitwise {
    /// Creates a new `Bitwise` instance wrapping the given `u64` value.
    ///
    /// # Arguments
    /// * `data` - The initial 64-bit value.
    ///
    pub fn new(data: u64) -> Self {
        Self { data }
    }

    /// Validates that a single bit index falls within the valid range `[0, 64)`.
    ///
    /// # Arguments
    /// * `index` - The bit index to validate.
    ///
    /// # Errors
    /// Returns an `Error` if `index > 63`, since a `u64` only has 64 valid
    /// bit positions.
    pub fn validate_index_range(index: u8) -> Result<(), Error> {
        if index > 63 {
            return Err(Error::other(
                "Index must be in the range of [0, 64) since it is the index of a bit of a 8 bytes integer!",
            ));
        }

        Ok(())
    }

    /// Validates that every index in a slice falls within the valid range `[0, 64)`.
    ///
    /// Unlike [`validate_index_range`](Self::validate_index_range), this collects
    /// *all* out-of-range indices before returning, rather than failing on the first one.
    ///
    /// # Arguments
    /// * `indicies` - A slice of bit indices to validate.
    ///
    /// # Errors
    /// Returns an `Error` listing all offending indices if any index is greater than 63.
    pub fn validate_indicies_range(indicies: &[u8]) -> Result<(), Error> {
        let mut outrange_indicies = vec![];

        for i in indicies {
            if let Err(_) = Self::validate_index_range(*i) {
                outrange_indicies.push(*i);
            }
        }

        if !outrange_indicies.is_empty() {
            return Err(Error::other(format!(
                "Index must be in the range of [0, 64) since it is the index of a bit of a 8 bytes integer!\nThe following indicies are outrange: {:?}
                ", outrange_indicies
            )));
        }

        Ok(())
    }

    /// Reads the value of the bit at the given index.
    ///
    /// # Arguments
    /// * `index` - The bit position to read (must be in `[0, 64)`).
    ///
    /// # Returns
    /// `Ok(1)` if the bit is set, `Ok(0)` if it is unset.
    ///
    /// # Errors
    /// Returns an `Error` if `index` is out of range (see
    /// [`validate_index_range`](Self::validate_index_range)).
    pub fn scan_bit(&self, index: u8) -> Result<u8, Error> {
        Self::validate_index_range(index)?;

        Ok(((self.data & (1 << index)) >> index) as u8)
    }

    /// Sets (to `1`) the bit at the given index.
    ///
    /// # Arguments
    /// * `index` - The bit position to set (must be in `[0, 64)`).
    ///
    /// # Errors
    /// Returns an `Error` if `index` is out of range.
    pub fn set_bit(&mut self, index: u8) -> Result<(), Error> {
        Self::validate_index_range(index)?;

        self.data |= 1 << index;

        Ok(())
    }

    /// Sets (to `1`) all bits at the given indices.
    ///
    /// # Arguments
    /// * `indicies` - A slice of bit positions to set (each must be in `[0, 64)`).
    ///
    /// # Errors
    /// Returns an `Error` listing any out-of-range indices; in that case
    /// no bits are modified.
    pub fn set_multiple_bits(&mut self, indicies: &[u8]) -> Result<(), Error> {
        Self::validate_indicies_range(indicies)?;

        let mut mask = 0;

        for i in indicies {
            mask |= mask | (1 << i);
        }

        self.data |= mask;

        Ok(())
    }

    /// Clears (sets to `0`) the bit at the given index.
    ///
    /// # Arguments
    /// * `index` - The bit position to clear (must be in `[0, 64)`).
    ///
    /// # Errors
    /// Returns an `Error` if `index` is out of range.
    pub fn clear_bit(&mut self, index: u8) -> Result<(), Error> {
        Self::validate_index_range(index)?;

        self.data = !(!self.data | (1 << index));

        Ok(())
    }

    /// Clears (sets to `0`) all bits at the given indices.
    ///
    /// # Arguments
    /// * `indicies` - A slice of bit positions to clear (each must be in `[0, 64)`).
    ///
    /// # Errors
    /// Returns an `Error` listing any out-of-range indices; in that case
    /// no bits are modified.
    pub fn clear_multiple_bits(&mut self, indicies: &[u8]) -> Result<(), Error> {
        Self::validate_indicies_range(indicies)?;

        for i in indicies {
            // cleared &= clear_bit(cleared, *i);
            self.data &= !(!self.data | (1 << i))
        }

        Ok(())
    }

    /// Flips (toggles) the bit at the given index: `0` becomes `1` and vice versa.
    ///
    /// # Arguments
    /// * `index` - The bit position to flip (must be in `[0, 64)`).
    ///
    /// # Errors
    /// Returns an `Error` if `index` is out of range.
    pub fn flip_bit(&mut self, index: u8) -> Result<(), Error> {
        Self::validate_index_range(index)?;

        self.data ^= 1 << index;

        Ok(())
    }

    /// Flips (toggles) all bits at the given indices.
    ///
    /// # Arguments
    /// * `indicies` - A slice of bit positions to flip (each must be in `[0, 64)`).
    ///
    /// # Errors
    /// Returns an `Error` listing any out-of-range indices; in that case
    /// no bits are modified.
    pub fn flip_multiple_bits(&mut self, indicies: &[u8]) -> Result<(), Error> {
        Self::validate_indicies_range(indicies)?;

        let mut mask = 0;

        for i in indicies {
            mask |= 1 << i;
        }

        self.data ^= mask;

        Ok(())
    }

    /// Renders the 64 bits as an 8x8 text grid (8 rows of 8 bits each),
    /// useful for visualizing bitboards (e.g., chess-style boards).
    ///
    /// Each cell is rendered as:
    /// - `" 1 "` if the bit is set,
    /// - `" . "` if the bit is unset,
    /// - `" X "` if its index equals `mark` (overriding the set/unset rendering).
    ///
    /// Rows are separated by newlines, with a new row starting every 8 bits.
    ///
    /// # Arguments
    /// * `mark` - An optional bit index to highlight with `" X "` instead of its actual value.
    ///
    /// # Returns
    /// A `String` containing the rendered 8x8 grid.
    ///
    /// # Panics
    /// Panics if an internal call to [`scan_bit`](Self::scan_bit) fails, which
    /// should not happen since indices `0..64` are always in range.
    ///
    /// # Note
    /// If `mark` is `None`, no bit values are rendered into the grid cells
    /// (the `Some(mark)` branch is required for any cell content to be pushed),
    /// so calling `to_string(None)` currently produces only blank rows.
    /// Pass `Some(index)` — even an out-of-board index like `64` — to render
    /// bit values without highlighting a specific cell.
    pub fn to_string(&self, mark: Option<u8>) -> String {
        let mut row = "".to_owned();
        let mut board = "".to_owned();

        for i in 0..64 as u8 {
            match self.scan_bit(i) {
                Ok(bit) => {
                    if let Some(mark) = mark {
                        if mark == i {
                            row.push_str(" X ");
                        } else {
                            if bit == 0 {
                                row.push_str(" . ")
                            } else {
                                row.push_str(" 1 ");
                            }
                        }
                    }
                }
                Err(e) => panic!("Error while pushing bit to row: {}", e),
            }

            if (i + 1).is_multiple_of(8) {
                row.push_str("\n");
            }

            board.push_str(&row);
            row.clear();
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_range() {
        assert!(Bitwise::validate_index_range(43).is_ok());
        assert!(Bitwise::validate_index_range(90).is_err());
    }

    #[test]
    fn test_indicies_range() {
        let indicies = [1, 2, 5];
        assert!(Bitwise::validate_indicies_range(&indicies).is_ok());

        let indicies = [100, 2, 65];
        assert!(Bitwise::validate_indicies_range(&indicies).is_err());
    }

    #[test]
    fn test_scan_bit() {
        let data = 0b1011_1110;

        let bitwise = Bitwise::new(data);
        let result = bitwise.scan_bit(0).unwrap();
        assert_eq!(result, 0);

        let bitwise = Bitwise::new(data);
        let result = bitwise.scan_bit(2).unwrap();
        assert_eq!(result, 1);

        let bitwise = Bitwise::new(data);
        let result = bitwise.scan_bit(3).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_set_bit() {
        let data = 0b1011_1010;

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.set_bit(6);
        assert_eq!(bitwise.data, 0b1111_1010);

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.set_bit(2);
        assert_eq!(bitwise.data, 0b1011_1110);
    }

    #[test]
    fn test_set_multiple_bits() {
        let data = 0b1001_1000;
        let indicies = [1, 2, 5];

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.set_multiple_bits(&indicies);
        assert_eq!(bitwise.data, 0b1011_1110);
    }

    #[test]
    fn test_flip_bit() {
        let data = 0b1011_1010;

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.flip_bit(2);
        assert_eq!(bitwise.data, 0b1011_1110);

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.flip_bit(5);
        assert_eq!(bitwise.data, 0b1001_1010);
    }

    #[test]
    fn test_multiple_bits() {
        let data = 0b1011_1010;
        let indicies = [2, 3, 5];

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.flip_multiple_bits(&indicies);
        assert_eq!(bitwise.data, 0b1001_0110)
    }

    #[test]
    fn test_clear_bit() {
        let data = 0b1011_1010;

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.clear_bit(1);
        assert_eq!(bitwise.data, 0b1011_1000);

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.clear_bit(3);
        assert_eq!(bitwise.data, 0b1011_0010);

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.clear_bit(5);
        assert_eq!(bitwise.data, 0b1001_1010);

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.clear_bit(6);
        assert_eq!(bitwise.data, data); // since 7th bit is 0 then the data and cleared_data will be the same
    }

    #[test]
    fn test_clear_multiple_bits() {
        let data = 0b1011_1010;
        let indicies = [1, 3, 5];

        let mut bitwise = Bitwise::new(data);
        let _ = bitwise.clear_multiple_bits(&indicies);
        assert_eq!(bitwise.data, 0b1001_0000);
    }
}
