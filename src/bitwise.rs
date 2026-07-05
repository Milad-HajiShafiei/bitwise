use std::io::Error;

pub struct Bitwise {
    pub data: u64,
}

impl Bitwise {
    pub fn new(data: u64) -> Self {
        Self { data }
    }

    pub fn validate_index_range(index: u8) -> Result<(), Error> {
        if index > 63 {
            return Err(Error::other(
                "Index must be in the range of [0, 64) since it is the index of a bit of a 8 bytes integer!",
            ));
        }

        Ok(())
    }

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

    /*
    Scan Bits (find the value or a bit) -> Scanning a bit using AND(&):
    Example: Find Value of Digit 3
         1011 1110 <- Data
        &0000 0100 <- Mask
         ---------
         0000 0100 <- Raw Result
         0000 0001 <- Shifted Result
    */
    pub fn scan_bit(&self, index: u8) -> Result<u8, Error> {
        Self::validate_index_range(index)?;

        Ok(((self.data & (1 << index)) >> index) as u8)
    }

    /*
    Set Bits (set the bit/bits to 1) -> Setting a bit using OR(|):
    Example: Set digit 7 to 1
         1011 1110 <- Data
        |0100 0000 <- Mask
         ---------
         1111 1110 <- Result
    */
    pub fn set_bit(&mut self, index: u8) -> Result<(), Error> {
        Self::validate_index_range(index)?;

        self.data |= 1 << index;

        Ok(())
    }

    pub fn set_multiple_bits(&mut self, indicies: &[u8]) -> Result<(), Error> {
        Self::validate_indicies_range(indicies)?;

        let mut mask = 0;

        for i in indicies {
            mask |= mask | (1 << i);
        }

        self.data |= mask;

        Ok(())
    }

    /*
    Clear Bits (set the bit/bits to 0) -> Clearing a digit to 0 using OR(|) and NOT(!):
    Example: Clear digit 5 to 0
         1111 0101 <- Data
        ! invert
         ---------
         0000 1010 <- Inverted
        |0001 0000 <- Mask
         ---------
         0001 1010 <- Masked
        ! invert
         ---------
         1110 0101 <- Result
    */
    pub fn clear_bit(&mut self, index: u8) -> Result<(), Error> {
        Self::validate_index_range(index)?;

        self.data = !(!self.data | (1 << index));

        Ok(())
    }

    pub fn clear_multiple_bits(&mut self, indicies: &[u8]) -> Result<(), Error> {
        Self::validate_indicies_range(indicies)?;

        for i in indicies {
            // cleared &= clear_bit(cleared, *i);
            self.data &= !(!self.data | (1 << i))
        }

        Ok(())
    }

    /*
    Toggle Bits (set a bit/bits to the binary opposite) -> Toggling/Flipping a bit using XOR(^):
    Example: Toggle digit 3
         1011 1110 <- Data
        ^0000 0100 <- Mask
         ---------
         1011 1010 <- Result
    */
    pub fn flip_bit(&mut self, index: u8) -> Result<(), Error> {
        Self::validate_index_range(index)?;

        self.data ^= 1 << index;

        Ok(())
    }

    pub fn flip_multiple_bits(&mut self, indicies: &[u8]) -> Result<(), Error> {
        Self::validate_indicies_range(indicies)?;

        let mut mask = 0;

        for i in indicies {
            mask |= 1 << i;
        }

        self.data ^= mask;

        Ok(())
    }

    // Display the data as a string
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
