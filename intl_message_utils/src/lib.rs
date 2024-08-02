/// Name of the JS runtime package that should be used for all generated code or parsing for imports
/// that read from the package.
pub static RUNTIME_PACKAGE_NAME: &str = "@discord/intl";

/// The seed used when computing hash keys for message names and other hashed identifiers.
pub static KEY_HASH_SEED: u64 = 0;

/// Lookup table used for quickly creating a base64 representation of a hashed key.
static BASE64_TABLE: &[u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

/// Returns a consistent, short hash of the given key by first processing it
/// through a sha256 digest, then encoding the first few bytes to base64.
pub fn hash_message_key(content: &str) -> String {
    let hash = xxhash_rust::xxh64::xxh64(content.as_bytes(), KEY_HASH_SEED);
    let input: [u8; 8] = hash.to_ne_bytes();
    // Since we know that we only want 6 characters out of the hash, we can
    // shortcut the base64 encoding to just directly read the bits out into an
    // encoded byte array and directly create a str from that.
    let output: Vec<u8> = vec![
        BASE64_TABLE[(input[0] >> 2) as usize],
        BASE64_TABLE[((input[0] & 0x03) << 4 | input[1] >> 4) as usize],
        BASE64_TABLE[((input[1] & 0x0f) << 2 | input[2] >> 6) as usize],
        BASE64_TABLE[(input[2] & 0x3f) as usize],
        BASE64_TABLE[(input[3] >> 2) as usize],
        BASE64_TABLE[((input[3] & 0x03) << 4 | input[3] >> 4) as usize],
    ];

    // SAFETY: We built this string out of ASCII characters, it doesn't need to
    // be checked for utf-8 validity.
    unsafe { String::from_utf8_unchecked(output) }
}

/// Returns true if the given file name is considered a message definitions file.
pub fn is_message_definitions_file(file_name: &str) -> bool {
    // `.messages` is the path used when importing, like:
    //     import {messages} from 'Somewhere.messages';
    file_name.ends_with(".messages")
    // All others are used when referencing the actual file path.
        || file_name.ends_with(".messages.tsx")
        || file_name.ends_with(".messages.jsx")
        || file_name.ends_with(".messages.ts")
        || file_name.ends_with(".messages.js")
}

pub fn is_message_translations_file(file_name: &str) -> bool {
    file_name.ends_with(".messages.json") || file_name.ends_with(".messages.jsona")
}
