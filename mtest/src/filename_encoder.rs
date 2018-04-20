use data_encoding::BASE64URL_NOPAD;
use std::fmt::Write;

pub fn encode_into_filename(id_str: &str) -> String {
    let id = id_str.as_bytes();
    let mut result = String::new();
    let mut escaped_seq_start = None;
    for (curr_idx, curr_byte) in id.iter().cloned().enumerate() {
        if byte_is_valid(curr_byte) {
            escape_seq(id, &mut escaped_seq_start, curr_idx, &mut result);
            result.push(curr_byte as char); // All valid chars are ASCII (1-byte UTF-8)
        } else {
            escaped_seq_start.get_or_insert(curr_idx);
        }
    }
    escape_seq(id, &mut escaped_seq_start, id.len(), &mut result);
    result
}

fn byte_is_valid(byte: u8) -> bool {
    BASE64URL_NOPAD.specification().symbols.as_bytes().contains(&byte)
        && byte != b'.'
}

fn escape_seq(id: &[u8], invalid_char_idx: &mut Option<usize>, curr_idx: usize, result: &mut String) {
    if let Some(idx) = invalid_char_idx.take() {
        write!(result, ".{}.", BASE64URL_NOPAD.encode(&id[idx..curr_idx])).unwrap()
    }
}
