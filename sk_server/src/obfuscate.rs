use anyhow::Result;
use base64::{self, prelude::*};
use std::sync::Mutex;

//pub static MASK: &[u8;8] = b"12345678";
//pub static MASK: Mutex<Vec<u8>>  = Mutex::new(vec![b'1',b'2',b'3',b'4', b'5', b'6', b'7', b'8']); // b"12345678".as_vec());
pub static MASK: Mutex<&[u8]>  = Mutex::new(b"12345678");


pub fn set_obfusication_mask(mask: &'static [u8]) {
    let mut mask_guard = MASK.lock().unwrap(); //expect("can not get lock on MASK");

    *mask_guard = mask;
}

/// Takes a key and a value, and returns the modified value as a string (which should be ascii, and probably base64)
pub fn mask_string(_key: &str, value: &str) -> Result<String> {
    //let mut byte_value = string.into_bytes();
    let mut byte_value = BASE64_STANDARD.decode(value)?;

    let mask = MASK.lock().unwrap();
    let mask_len = mask.len();

    // apply the mask to the bytes
    byte_value.iter_mut().enumerate().for_each(|(idx,v)| { *v = *v ^ mask[idx % mask_len]; });


    Ok(BASE64_STANDARD.encode(byte_value))
}

#[cfg(test)]
mod test {

use crate::obfuscate::{mask_string, set_obfusication_mask};
use base64::prelude::*;

    const KEY: &str = "no_relevant_yet";

    fn init_tests() {
        set_obfusication_mask(b"12345678");
    }

    #[test]
    fn round_trip_short_string() {
        // init_tests();

        let input = BASE64_STANDARD.encode("Hello");

        let masked = mask_string(KEY, &input).unwrap();

        let restored = mask_string(KEY, &masked).unwrap();

        assert_eq!(input, restored)
    }


    #[test]
    fn round_trip_long_string() {
        // init_tests();
        
        let input = BASE64_STANDARD.encode("Hello World, here I am again, but this time with a string that exceeds the mask");

        let masked = mask_string(KEY, &input).unwrap();

        let restored = mask_string(KEY, &masked).unwrap();

        assert_eq!(input, restored)
    }

    #[test]
    fn check_short_string() {
        init_tests();
        
        let input = BASE64_STANDARD.encode("Hello World!");

        let masked = mask_string(KEY, &input).unwrap();

        let expected: &[u8] = &[('H' as u8 ^ '1' as u8),
                                    'e' as u8 ^ '2' as u8 ,
                                    'l' as u8 ^ '3' as u8 ,
                                    'l' as u8 ^ '4' as u8 ,
                                    'o' as u8 ^ '5' as u8 ,
                                    ' ' as u8 ^ '6' as u8 ,
                                    'W' as u8 ^ '7' as u8 ,
                                    'o' as u8 ^ '8' as u8 ,
                                    'r' as u8 ^ '1' as u8 ,
                                    'l' as u8 ^ '2' as u8 ,
                                    'd' as u8 ^ '3' as u8 ,
                                    '!' as u8 ^ '4' as u8 ];
        let result = BASE64_STANDARD.decode(masked).unwrap();
        assert_eq!(result, expected, " result '{result:?}' does not match the expected value '{expected:?}'."); 
    }
}