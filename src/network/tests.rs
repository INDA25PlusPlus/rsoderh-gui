use super::*;

#[test]
fn read_until_slice_standard() {
    let mut reader = BufReader::new(b"garbagePrefix more garbage".as_slice());

    assert_eq!(skip_until_slice(&mut reader, b"Prefix").unwrap(), Some(()),);

    let mut result = vec![];
    reader.read_to_end(&mut result).unwrap();
    assert_eq!(result.as_slice(), b"Prefix more garbage",)
}

#[test]
fn read_until_slice_long_prefix() {
    let mut reader = BufReader::new(b"shortLongPrefix".as_slice());

    assert_eq!(
        skip_until_slice(&mut reader, b"LongPrefix").unwrap(),
        Some(()),
    );

    let mut result = vec![];
    reader.read_to_end(&mut result).unwrap();
    assert_eq!(result.as_slice(), b"LongPrefix",)
}

#[test]
fn read_until_slice_repeated() {
    let mut reader = BufReader::new(b"garbagePrefix".as_slice());

    assert_eq!(skip_until_slice(&mut reader, b"Prefix").unwrap(), Some(()),);
    assert_eq!(skip_until_slice(&mut reader, b"Prefix").unwrap(), Some(()),);

    let mut result = vec![];
    reader.read_to_end(&mut result).unwrap();
    assert_eq!(result.as_slice(), b"Prefix",)
}
