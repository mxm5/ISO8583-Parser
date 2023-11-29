fn positions_of_set_bits(n: u64) -> Vec<u32> {
    (0..64).filter(|&bit| 1 & (n >> (63 - bit)) != 0).map(|bit| bit + 1).collect()
}
trait StringManipulation {
    fn get_slice_until(&mut self, length: usize) -> String;
}
impl StringManipulation for String {
    fn get_slice_until(&mut self, length: usize) -> String {
        self.drain(..length).collect::<String>()
    }
}

fn main() {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Please enter a message to parse: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    s = s.replace("\"", "").replace(" ", "");

    println!("Lentgh Of Message: {:?}", u32::from_str_radix(&s.get_slice_until(4), 16).expect("Unable to get the length"));
    println!("Non Parsed Yet: {:?}", s.get_slice_until(10));
    println!("MTI: {:?}", s.get_slice_until(4));
    let bitmap = positions_of_set_bits(u64::from_str_radix(&s.get_slice_until(16),16).expect("Unable to get the process code"));
    println!("First Bit Map: {:?}", bitmap);


    for bit in bitmap {
        match bit {
            3 => println!("Process Code: {}", s.get_slice_until(6)),
            4 => println!("Amount: {:?}", s.get_slice_until(12)),
            11 => println!("Trace: {:?}", s.get_slice_until(6)),
            12 => println!("Time: {:?}", s.get_slice_until(6)),
            13 => println!("Date: {:?}", s.get_slice_until(4)),
            22 => println!("Field22: {:?}", s.get_slice_until(4)),
            24 => println!("Field24: {:?}", s.get_slice_until(4)),
            25 => println!("Field25: {:?}", s.get_slice_until(2)),
            35 => {
                let track2_len: u32 = u32::from_str_radix(&s.get_slice_until(2), 10).expect("Unable to get the length of track2");
                println!(" Field35 length: {:?}",track2_len);
                println!("Track2: {:?}", s.get_slice_until(38));
            }
            41 => println!("Terminal: {:?}", s.get_slice_until(16)),
            42 => println!("Acceptor: {:?}", s.get_slice_until(30)),
            48 => { 
                let filed48_len: u32 = u32::from_str_radix(&s.get_slice_until(4), 10).expect("Unable to get the length of field48") * 2;
                println!(" Filed48 length: {:?}", filed48_len);
                println!("Filed48: {:?}", s.get_slice_until(filed48_len as usize));
                }
            49 => println!("Currency: {:?}", s.get_slice_until(6)),
            52 => println!("Filed52 PinBlock {:?}", s.get_slice_until(16)),
            55 => { 
                let filed55_len: u32 = u32::from_str_radix(&s.get_slice_until(4), 10).expect("Unable to get the length of field55") * 2;
                println!(" Filed55 length: {:?}", filed55_len);
                println!("Filed55: {:?}", s.get_slice_until(filed55_len as usize));
                }
            62 => { 
                let filed62_len: u32 = u32::from_str_radix(&s.get_slice_until(4), 10).expect("Unable to get the length of field62") * 2;
                println!(" Filed62 length: {:?}", filed62_len);
                println!("Filed62: {:?}", s.get_slice_until(filed62_len as usize));
                }                
            64 =>println!("MAC: {:?}", s.get_slice_until(16)),
            num => println!("The number {} is not defined yet.", num),
        }
    }
    if !s.is_empty() {
        println!("Not parsed Part: {:?}",s);
    }
}