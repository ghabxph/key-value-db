
/// Split string by given split character
pub fn split(string: &[u8], split_char: u8) -> Vec<Vec<u8>> {
    let mut strings = Vec::<Vec<u8>>::new();
    let mut k = 0;
    let eol = string.len() - 1;
    for i in 0..string.len() {
        if string[i] == split_char || i == eol {
            let mut substring = Vec::<u8>::new();
            if i == eol && string[i] != split_char {
                for j in k..eol + 1 {
                    substring.push(string[j]);
                }
            } else {
                for j in k..i {
                    substring.push(string[j]);
                }
                k = i + 1;
            }
            strings.push(substring);
        }
    }
    return strings;
}

#[test]
pub fn test_split() {
    let string = b"hello\x0Aworld\x0AMUNDO\x0A";
    let strings = split(string, b'\x0A');
    assert!(strings.len() == 3);
    assert!(strings.get(0).unwrap().eq(b"hello"));
    assert!(strings.get(1).unwrap().eq(b"world"));
    assert!(strings.get(2).unwrap().eq(b"MUNDO"));
}