use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

use crate::error::PlerionError;

/// Every ASCII character `encodeURIComponent` escapes, so this client and the
/// Pleri client encode the same id identically. `encodeURIComponent` leaves only
/// `A-Z a-z 0-9 - _ . ! ~ * ' ( )` unescaped; everything else is listed here.
/// Kept honest by `test_encoding_matches_encode_uri_component`.
const PATH_SEGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'+')
    .add(b',')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

/// Percent-encodes an id for use as a single path segment. `label` names the
/// id in the error, e.g. "grant ID".
///
/// An empty id is rejected, since it would call the parent collection route.
///
/// A segment of only dots is rejected rather than encoded. It is an RFC 3986
/// dot-segment, and the URL layer resolves it before the request is sent, so
/// `..` would walk up a path level and silently call a different endpoint.
/// Encoding does not help: `%2E` is decoded and normalised the same way, so
/// there is no way to express a literal dot-segment. Refusing is the only
/// correct option, and no real id looks like this.
pub(crate) fn segment(id: &str, label: &str) -> Result<String, PlerionError> {
    if id.bytes().all(|b| b == b'.') {
        return Err(PlerionError::InvalidArgument(format!(
            "'{id}' is not a valid {label}"
        )));
    }
    Ok(utf8_percent_encode(id, PATH_SEGMENT).to_string())
}

#[cfg(test)]
mod tests {
    use super::segment;

    fn seg(id: &str) -> Result<String, crate::error::PlerionError> {
        segment(id, "ID")
    }

    /// The only ASCII characters `encodeURIComponent` leaves unescaped.
    const UNRESERVED: &str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.!~*'()";

    /// Pins the comment on PATH_SEGMENT: escape exactly what encodeURIComponent
    /// escapes, so this client and the Pleri client encode an id identically.
    /// A dot-only segment is the one exception, covered separately below.
    #[test]
    fn test_encoding_matches_encode_uri_component() {
        for byte in 0x20u8..0x7f {
            let ch = char::from(byte);
            if ch == '.' {
                continue;
            }
            let input = ch.to_string();
            let escaped = seg(&input).unwrap() != input;
            assert_eq!(
                escaped,
                !UNRESERVED.contains(ch),
                "{ch:?} (0x{byte:02x}) escaped={escaped}, encoded as {}",
                seg(&input).unwrap()
            );
        }
    }

    #[test]
    fn test_control_characters_are_escaped() {
        assert_eq!(seg("\n").unwrap(), "%0A");
        assert_eq!(seg("\t").unwrap(), "%09");
    }

    #[test]
    fn test_a_uuid_passes_through_unchanged() {
        let id = "0f9a1c3e-5b7d-4c21-9e8f-2a6b4d10c7f3";
        assert_eq!(seg(id).unwrap(), id);
    }

    /// A dot-only segment cannot be expressed in a URL, so it is refused rather
    /// than silently resolving to a different endpoint.
    #[test]
    fn test_dot_segments_are_rejected() {
        assert!(seg("").is_err());
        assert!(seg(".").is_err());
        assert!(seg("..").is_err());
        assert!(seg("...").is_err());
        // Dots inside a real value are fine.
        assert_eq!(seg("a.b").unwrap(), "a.b");
        assert_eq!(seg(".hidden").unwrap(), ".hidden");
    }

    #[test]
    fn test_error_names_the_label() {
        let err = segment("..", "risk ID").unwrap_err().to_string();
        assert_eq!(err, "'..' is not a valid risk ID");
    }
}
