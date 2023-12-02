use emv_tlv_parser::parse_tlv;

fn positions_of_set_bits(n: u64) -> Vec<u32> {
    (0..64).filter(|&bit| 1 & (n >> (63 - bit)) != 0).map(|bit| bit + 1).collect()
}

trait StringManipulation {
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

fn read_data_from_stdin()-> String {
    use std::io::{stdin,stdout,Write};
    let mut data_raw =String::new();
    print!("Please enter a message to parse: ");
    let _=stdout().flush();
    stdin().read_line(&mut data_raw).expect("Did not enter a correct string");
    if let Some('\n')=data_raw.chars().next_back() {
        data_raw.pop();
    }
    if let Some('\r')=data_raw.chars().next_back() {
        data_raw.pop();
    }
    data_raw
}


fn main() {
    let mut s= read_data_from_stdin();

    s = s.replace("\"", "").replace(" ", "");

    println!("Lentgh Of Message: {}", u32::from_str_radix(&s.get_slice_until(4), 16).expect("Unable to get the length"));
    println!("Non Parsed Yet: {}", s.get_slice_until(10));
    println!("MTI: {}", s.get_slice_until(4));
    let bitmap: Vec<u32> = positions_of_set_bits(u64::from_str_radix(&s.get_slice_until(16),16).expect("Unable to get the process code"));
    println!("First Bit Map: {:?}", bitmap);

    for &bit in &bitmap {
        match bit {
            3 => s.process_field(3, 6, "Process Code"),
            4 => s.process_field(4, 12, "Amount"),
            11 => s.process_field(11, 6, "Trace"),
            12 => s.process_field(12, 6, "Time"),
            13 => s.process_field(13, 4, "Date"),
            22 => s.process_field(22, 4, ""),
            24 => s.process_field(24, 4, ""),
            25 => s.process_field(25, 2, ""),
            35 => {
                let track2_len: u32 = s.get_slice_until(2).parse::<u32>().unwrap();
                s.process_field(35, track2_len, "Track2");
            }
            41 => s.process_field(41, 16, "Terminal"),
            42 => s.process_field(42, 30, "Acceptor"),
            48 => {
                let field48_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(48, field48_len, "");
            }
            49 => s.process_field(49, 6, "Currency"),
            52 => s.process_field(52, 16, "PinBlock"),
            55 => {
                let field55_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(55, field55_len, "");
            }
            62 => {
                let field62_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(62, field62_len, "");
            }
            64 => s.process_field(64, 16, "MAC"),
            num => println!("The number {} is not defined yet.", num),
        }
    }
    if !s.is_empty() {
        println!("Not parsed Part: {}",s);
    }
}