#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_through_base64() {
        unsafe {
            let input = include_str!("../futurama-quotes.txt").as_bytes();
            let mut encoded_output: Vec<u8> = vec![0; input.len()*2];
            let mut unencoded_output: Vec<u8> = vec![0; input.len()];

            apr_base64_encode(encoded_output.as_mut_ptr() as *mut _,
                              input.as_ptr() as *const _,
                              input.len() as i32);

            let unencoded_len = apr_base64_decode(unencoded_output.as_mut_ptr() as *mut _,
                                                  encoded_output.as_ptr() as *const _);

            assert_ne!(encoded_output, unencoded_output);
            assert_eq!(unencoded_len, input.len() as i32);
            assert_eq!(input, &unencoded_output[..]);
        }
    }
}
