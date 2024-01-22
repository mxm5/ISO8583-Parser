use iso8583_parser::{StringManipulation, positions_of_set_bits};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message() {
        let test_message = 
        "00A8600008000002003038058020C1920100000000000000000100275516040011250021000800388362143741176336D28112011861000000000F303131343430323130313132393633302020202020202000321101313131313131313131311002312E312E312E312E31020330022231021532333634000000000000000000335F2A02036482020800950580000000009A032311259C01009F02060000000000011111111111111111";

        let mut s = test_message.to_string();
        assert_eq!(s.get_slice_until(4), "00A8");
        assert_eq!(s.get_slice_until(10), "6000080000");
        let mti = s.get_slice_until(4);
        assert_eq!(mti, "0200");

        let bitmap: Vec<u32> = positions_of_set_bits(u64::from_str_radix(&s.get_slice_until(16), 16).expect("Unable to get the process code"));
        assert_eq!(bitmap, vec![3, 4, 11, 12, 13, 22, 24, 25, 35, 41, 42, 48, 49, 52, 55, 64]);

        assert_eq!(s.get_slice_until(6), "000000");
        assert_eq!(s.get_slice_until(12), "000000000001");
        assert_eq!(s.get_slice_until(6), "002755");
        assert_eq!(s.get_slice_until(6), "160400");

    }
    // Add more tests for other functions and methods
}