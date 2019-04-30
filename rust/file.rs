#[test]
fn test_simple_file() {
    let path = "./tmp/test_simple_file.txt";

    fs::write(path, "a\nb\nc");

    // Rust raw fs call syscall each instruction.
    // Should use BufReader or BufWriter due to its bad performance.

    let mut f = OpenOptions::new()
      .read(true)
      .write(true)
      .open(path)
      .unwrap();

    // position moves after each instruction.

    let mut s = String::new();
    f.seek(SeekFrom::Start(0));
    f.read_to_string(&mut s);
    assert_eq!(s, "a\nb\nc");

    let mut v = vec![];
    f.seek(SeekFrom::Start(0));
    f.read_to_end(&mut v);
    assert_eq!(v, b"a\nb\nc");

    let mut s = String::new();
    f.seek(SeekFrom::Start(0));
    f.write(b"z").unwrap();
    f.seek(SeekFrom::Start(0));
    f.read_to_string(&mut s);
    assert_eq!(s, "z\nb\nc");
}

#[test]
fn test_buf_file() {
    let path = "./tmp/test_buf_file.txt";

    fs::write(path, "a\nb\nc");

    let mut f = OpenOptions::new()
      .read(true)
      .write(true)
      .open(path)
      .unwrap();

    let mut reader = BufReader::new(&f);
    let mut writer = BufWriter::new(&f);

    // position moves after each instruction.

    let mut s = String::new();
    reader.read_to_string(&mut s);
    assert_eq!(s, "a\nb\nc");

    let mut v = vec![];
    reader.seek(SeekFrom::Start(0));
    reader.read_to_end(&mut v);
    assert_eq!(v, b"a\nb\nc");

    let mut s = String::new();
    writer.seek(SeekFrom::Start(0));
    writer.write(b"z").unwrap();
    writer.seek(SeekFrom::Start(0));
    reader.read_to_string(&mut s);
    assert_eq!(s, "z\nb\nc");
}
