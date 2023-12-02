pub mod string_manipulation {
    use emv_tlv_parser::parse_tlv;

    pub fn positions_of_set_bits(n: u64) -> Vec<u32> {
        (0..64).filter(|&bit| 1 & (n >> (63 - bit)) != 0).map(|bit| bit + 1).collect()
    }
    
    pub trait StringManipulation {
        fn get_slice_until(&mut self, length: usize) -> String;
        fn hex_to_ascii(&mut self) -> Result<String, hex::FromHexError>;
        fn process_field(&mut self, field_number :u32, length :u32, name: &str);
    }

    impl StringManipulation for String {

        fn get_slice_until(&mut self, length: usize) -> String {
            self.drain(..length).collect::<String>()
        }

        fn hex_to_ascii(&mut self) -> Result<String, hex::FromHexError> {
            let hex_bytes = hex::decode(self)?;
            let ascii_chars: String = hex_bytes.iter().map(|&byte| byte as char).collect();
            Ok(ascii_chars)
        }

        fn process_field(&mut self, field_number :u32, length :u32, name: &str) {

            let mut field_value = if field_number == 35 {
                self.get_slice_until(38 as usize)
            } else {
                self.get_slice_until(length as usize)
            };

            let value_to_print = if matches!(field_number, 41 | 42 | 49 | 62) {
                field_value.hex_to_ascii().unwrap()
            } else {
                field_value.to_string()
            };

            println!("Field {:3} | {:12} | Length: {:3} | {}", field_number, name, length, value_to_print);
            
            if field_number == 55 {
                match parse_tlv(value_to_print) {
                    Ok(tags) => tags.iter().for_each(|tag| println!("{}", tag)),
                    Err(e) => eprintln!("Error parsing TLV: {}", e),
                }
            }
        }
    }
}