use simple_rs_sha2_384::{digest, sha2_384};

const TESTS: &[(&str, &str)] = &[
    ("", "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b"),
    ("hello", "59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f"),
    ("abc", "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"),
    ("zxcv", "f6eebea0fba46e63f066da0a895e60b46d5c3229d385529a29fe7c15caf8bc3d08624a261178a74eefd8c94f54b40131"),
    ("The quick brown fox jumps over the lazy dog", "ca737f1014a48f4c0b6dd43cb177b0afd9e5169367544c494011e3317dbf9a509cb1e5dc1e85a941bbee3d7f2afbc9b1"),
    ("The quick brown fox jumps over the lazy dog.", "ed892481d8272ca6df370bf706e4d7bc1b5739fa2177aae6c50e946678718fc67a7af2819a021c2fc34e91bdb63409d7"),
    ("1234567890qwertyuiopasdfghjklzxcvbnm1234567890qwertyuiopasdfghjklzxcvbnm", "f3ac4012f95e266d07a84ce326233b4f23a81a7aca684a318633de001a59a0b38f509b6167efa5064eae1d5c4c37e8ec"),
    ("abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu", "09330c33f71147e83d192fc782cd1b4753111b173b3b05d22fa08086e3b0f712fcc7c71a557e2db966c3e9fa91746039"),
    ("1abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu", "623289cc05b54a7f02827a4fb1d12c4d9ded0d55e245bda44118beb2b2db67b807333828789676e80fd2c1af26199673"),
    ("héllo wörld 🦀", "220b17f214e5f02f05468307526fbcc806a59b83bb0e72202ffa09223b9956df23665012080cff0c5206b113619f2adb"),
];

#[test]
fn known_vectors() {
    for (input, expected) in TESTS {
        assert_eq!(&sha2_384(input), expected, "input: {input:?}");
    }
}

/// Every length around the padding boundaries (one and two final blocks).
#[test]
fn all_lengths_up_to_300() {
    let expected = include_str!("lengths.txt");
    let data: Vec<u8> = (0..300u32).map(|i| (i * 31 + 7) as u8).collect();
    for (len, line) in expected.lines().enumerate() {
        assert_eq!(sha2_384(&data[..len]), line, "length {len}");
    }
}

#[test]
fn bytes_and_str_agree() {
    assert_eq!(
        sha2_384("hello"),
        sha2_384(String::from("hello").into_bytes())
    );
    assert_eq!(digest("hello").len(), 48);
}
