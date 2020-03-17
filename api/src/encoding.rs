use std::borrow::Cow;

use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

const ENCODING_SET: &'static AsciiSet = &NON_ALPHANUMERIC
    .remove(b';')
    .remove(b',')
    .remove(b'/')
    .remove(b'?')
    .remove(b':')
    .remove(b'@')
    .remove(b'&')
    .remove(b'=')
    .remove(b'+')
    .remove(b'$')
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'!')
    .remove(b'~')
    .remove(b'*')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'#');

pub fn encode_uri(u: &str) -> Cow<'_, str> {
    utf8_percent_encode(u, ENCODING_SET).into()
}
