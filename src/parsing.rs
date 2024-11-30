pub mod tools {
    extern crate minreq;
    use std::borrow::Cow;
    use std::error::Error;

    // Assumes a nice structured JSON, with no spaces etc...
    pub fn parse_json_value<'a>(
        json_bytes: &'a [u8],
        key_bytes: &[u8],
        position: &mut usize,
    ) -> Option<Cow<'a, str>> {
        // Find the position of the key in the byte slice (no allocations)
        if let Some(key_start) = efficient_find_bytes(json_bytes, key_bytes, *position) {
            // Move past the key and colon
            let value_start = *position + key_start + key_bytes.len();

            // Try to find where the value ends (looking for comma or closing brace)
            let value_end = json_bytes[value_start..]
                .iter()
                .position(|&b| b == b',' || b == b'}')
                .unwrap_or(json_bytes.len() - value_start);

            let value_bytes = &json_bytes[value_start..value_start + value_end];

            // Next start position for a search
            *position = value_start + value_end + 1;

            // If the value is wrapped in quotes, strip them
            if value_bytes.starts_with(&[b'"']) && value_bytes.ends_with(&[b'"']) {
                return Some(Cow::Borrowed(
                    std::str::from_utf8(&value_bytes[1..value_bytes.len() - 1]).unwrap(),
                ));
            }
            Some(Cow::Borrowed(std::str::from_utf8(value_bytes).unwrap()))
        } else {
            None
        }
    }

    /*
    Find the position of a byte slice inside (needle) in another byte slice (haystack)
    fn naive_find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
        // If the needle is empty, return the start position (0)
        if needle.is_empty() {
            return Some(0);
        }

        // Loop through the haystack and check each possible starting position
        for i in start..=haystack.len() - needle.len() {
            let mut match_found = true;

            // Compare the slice of the haystack starting at position i with the needle
            for j in 0..needle.len() {
                if haystack[i + j] != needle[j] {
                    match_found = false;
                    break; // If there's a mismatch, stop comparing and move to the next position
                }
            }

            if match_found {
                return Some(i-start); // Return the position if a match is found
            }
        }

        None // Return None if no match is found
    }
    */

    // Find the position of a byte (needle) in another byte slice (haystack)
    // TODO: A SIMD approach could be used to compare e.g. 32 bytes at once
    fn efficient_find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
        haystack[start..]
            .windows(needle.len())
            .position(|window| window == needle)
    }
}
