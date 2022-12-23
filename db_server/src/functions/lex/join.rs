
/// Join strings by given join character
pub fn join(strings: Vec<Vec<u8>>, join_char: u8) -> Vec<u8> {
    let mut joined = Vec::<u8>::new();
    for i in 0..strings.len() {
        let substring = strings.get(i).unwrap();
        for j in 0..substring.len() {
            joined.push(*substring.get(j).unwrap());
        }
        if i != strings.len() - 1 {
            joined.push(join_char);
        }
    }
    return joined;
}

#[test]
pub fn test_join() {
    let hello = b"hello";
    let mundo = b"mundo";
    let join_char = b'\x0A';
    let mut strings = Vec::<Vec<u8>>::new();
    strings.push(hello.to_vec());
    strings.push(mundo.to_vec());
    let result = join(strings, join_char);
    assert!(result.len() == 11);
    assert!(result.eq(b"hello\x0Amundo"));
}