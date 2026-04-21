use anyhow::Result;
use base64::{self, prelude::*};


pub static MASK: &[u8;8] = b"12345678";


/// Takes a key and a value, and returns the modified value as a string (which should be ascii, and probably base64)
pub fn mask_string(key: &str, value: &str) -> Result<String> {
    //let mut byte_value = string.into_bytes();
    let mut byte_value = BASE64_STANDARD.decode(value)?;
    // apply the mask to the bytes
    byte_value.iter_mut().enumerate().for_each(|(idx,v)| { *v = *v ^ MASK[idx % 8]; });


    Ok(BASE64_STANDARD.encode(byte_value))
}

#[cfg(test)]
mod test {

use crate::obfusicate::mask_string;
use base64::prelude::*;

    const KEY: &str = "no_relevant_yet";

    #[test]
    fn round_trip_short_string() {
        let input = BASE64_STANDARD.encode("Hello");

        let masked = mask_string(KEY, &input).unwrap();

        let restored = mask_string(KEY, &masked).unwrap();

        assert_eq!(input, restored)
    }


    #[test]
    fn round_trip_long_string() {
        let input = BASE64_STANDARD.encode("Hello World, here I am again, but this time with a string that exceeds the mask");

        let masked = mask_string(KEY, &input).unwrap();

        let restored = mask_string(KEY, &masked).unwrap();

        assert_eq!(input, restored)
    }

    #[test]
    fn check_short_string() {
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