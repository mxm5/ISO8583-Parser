use iso8583_parser::{StringManipulation, positions_of_set_bits};
use clap::Parser;

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

/// Arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// message to get
    #[arg(short, long, required = false)]
    message: Option<String>,

    #[arg(short, long)]
    including_header_length: bool,
}

fn main() {
    // Get command-line arguments
    let args = Args::parse();

    // Check if message argument is provided unless read data from stdin
    let mut s = match args.message {
        Some(m) => m,
        None => read_data_from_stdin(), 
    };
    s = s.replace("\"", "").replace(" ", "");
    if args.including_header_length {
        let message_len = u32::from_str_radix(&s.get_slice_until(4), 16).expect("Unable to get the length") * 2;
        println!("Lentgh Of Message: {}", message_len );
        if s.len() != message_len as usize {
            panic!("Error: Incorrect message len. The expected length is {} but The actual is {}", message_len,  s.len());
        }
        println!("Header: {}", s.get_slice_until(10));
    }
    println!("MTI: {}", s.get_slice_until(4));
    let mut bitmap: Vec<u32> = positions_of_set_bits(u64::from_str_radix(&s.get_slice_until(16),16).expect("Unable to get the process code"));
    if bitmap.contains(&1) {
        let mut positions = positions_of_set_bits(u64::from_str_radix(&s.get_slice_until(16), 16).expect("Unable to get the process code"));
        positions.iter_mut().for_each(|num| *num += 64);
        bitmap.append(&mut positions);
        bitmap.retain(|&x| x != 1);
    }
    println!("First Bit Map: {:?}", bitmap);

    for &bit in &bitmap {
        match bit {
            2 => {
                let pan_len: u32 =  s.get_slice_until(2).parse::<u32>().unwrap();
                s.process_field(2, pan_len, "PAN");
            }
            3 => s.process_field(3, 6, "Process Code"),
            4 => s.process_field(4, 12, "Transaction Amount"),
            5 => s.process_field(5, 12, "Settlement Amount"),
            6 => s.process_field(6, 12, "Cardholder Billing Amount"),
            7 => s.process_field(7, 10, "Transaction Date and Time"),
            11 => s.process_field(11, 6, "Trace"),
            12 => s.process_field(12, 6, "Time"),
            13 => s.process_field(13, 4, "Date"),
            14 => s.process_field(14, 4, "Card EXpiration Date"),
            18 => s.process_field(18, 4, "Merchant Category Code"),
            19 => s.process_field(19, 3, "Acquirer Country Code"),
            22 => s.process_field(22, 4, "POS Entry Mode"),
            23 => s.process_field(23, 3, "Card Sequence Number"),
            24 => s.process_field(24, 4, ""),
            25 => s.process_field(25, 2, ""),
            35 => {
                let track2_len: u32 = s.get_slice_until(2).parse::<u32>().unwrap();
                s.process_field(35, track2_len, "Track2");
            }
            37 => s.process_field(37, 24, "Retrieval Ref #"),
            38 => s.process_field(38, 12, "Authorization Code"),
            39 => s.process_field(39, 4, "Response Code"),
            41 => s.process_field(41, 16, "Terminal"),
            42 => s.process_field(42, 30, "Acceptor"),
            43 => s.process_field(43, 40, "Card Acceptor Name/Location"),
            45 => {
                let track1_len: u32 = s.get_slice_until(2).parse::<u32>().unwrap();
                s.process_field(45, track1_len, "Track 1 Data");
            }
            48 => {
                let field48_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(48, field48_len, "Aditional Data");
            }
            49 => s.process_field(49, 6, "Transaction Currency Code"),
            50 => s.process_field(50, 6, "Settlement Currency Code"),
            51 => s.process_field(51, 6, "Billing Currency Code"),
            52 => s.process_field(52, 16, "PinBlock"),
            54 => {
                let field54_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(54, field54_len, "Amount");
            }
            55 => {
                let field55_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(55, field55_len, "");
            }
            60 => {
                let field60_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(60, field60_len, "");              
            }
            62 => {
                let field62_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(62, field62_len, "IP And Port");
            }
            64 => s.process_field(64, 16, "MAC"),
            70 => s.process_field(70, 4, ""),
            122 => {
                let field122_len = s.get_slice_until(4).parse::<u32>().unwrap() * 2;
                s.process_field(122, field122_len, "Additional Data");
            }
            128 => s.process_field(128, 16, "MAC"),
            num => {println!("The number {} is not defined yet.", num); return;},
        }
    }
    if !s.is_empty() {
        println!("Not parsed Part: {}",s);
    }
}

