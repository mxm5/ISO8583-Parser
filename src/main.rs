use emv_parser::{StringManipulation, positions_of_set_bits};

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
    let mut s: String;

    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if at least one argument is provided
    if args.len() > 1 {
        // Argument provided, use the first one
         s = args[1].clone();
        println!("Argument: {}", s);
    } else {
        // No argument provided, use a default message
        println!("No argument provided. Enter a message:");
        s = read_data_from_stdin();
    }

    s = s.replace("\"", "").replace(" ", "");
    let message_len = u32::from_str_radix(&s.get_slice_until(4), 16).expect("Unable to get the length") * 2;
    println!("Lentgh Of Message: {}", message_len );
    if s.len() != message_len as usize {
        panic!("Error: Incorrect message len. The expected length is {} but The actual is {}", message_len,  s.len());
    }
    println!("Non Parsed Yet: {}", s.get_slice_until(10));
    println!("MTI: {}", s.get_slice_until(4));
    let bitmap: Vec<u32> = positions_of_set_bits(u64::from_str_radix(&s.get_slice_until(16),16).expect("Unable to get the process code"));
    println!("First Bit Map: {:?}", bitmap);

    for &bit in &bitmap {
        match bit {
            2 => {
                let pan_len: u32 =  s.get_slice_until(2).parse::<u32>().unwrap();
                s.process_field(2, pan_len, "PAN");
            }
            3 => s.process_field(3, 6, "Process Code"),
            4 => s.process_field(4, 12, "Amount"),
            11 => s.process_field(11, 6, "Trace"),
            12 => s.process_field(12, 6, "Time"),
            13 => s.process_field(13, 4, "Date"),
            14 => s.process_field(14, 4, "Card EXpiration Date"),
            22 => s.process_field(22, 4, "POS Entry Mode"),
            23 => s.process_field(23, 3, "Card Sequence Number"),
            24 => s.process_field(24, 4, ""),
            25 => s.process_field(25, 2, ""),
            35 => {
                let track2_len: u32 = s.get_slice_until(2).parse::<u32>().unwrap();
                s.process_field(35, track2_len, "Track2");
            }
            41 => s.process_field(41, 16, "Terminal"),
            42 => s.process_field(42, 30, "Acceptor"),
            45 => {
                let track1_len: u32 = s.get_slice_until(2).parse::<u32>().unwrap();
                s.process_field(45, track1_len, "Track 1 Data"),
            }
            48 => {
                let field48_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(48, field48_len, "Aditional Data");
            }
            49 => s.process_field(49, 6, "Currency"),
            52 => s.process_field(52, 16, "PinBlock"),
            55 => {
                let field55_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(55, field55_len, "");
            }
            62 => {
                let field62_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(62, field62_len, "IP And Port");
            }
            64 => s.process_field(64, 16, "MAC"),
            num => println!("The number {} is not defined yet.", num),
        }
    }
    if !s.is_empty() {
        println!("Not parsed Part: {}",s);
    }
}

