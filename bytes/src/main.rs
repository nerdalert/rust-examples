use bytes::{BufMut, Bytes, BytesMut};

fn main() {
    // let mut buf = BytesMut::with_capacity(1024);
    // buf.put(&b"hello world"[..]);
    // buf.put_u16(1234);
    // let a = buf.split();
    // assert_eq!(a, b"hello world\x04\xD2"[..]);

    // buf.put(&b"goodbye world"[..]);
    // let b = buf.split();
    // assert_eq!(b, b"goodbye world"[..]);
    // assert_eq!(buf.capacity(), 998);

    // Example 2
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    buf.put_u16(1234);
    let a = buf.split();
    assert_eq!(a, b"hello world\x04\xD2"[..]);
    buf.put(&b"goodbye world"[..]);
    let b = buf.split();
    assert_eq!(b, b"goodbye world"[..]);
    assert_eq!(buf.capacity(), 998);

    // Example 3
    let b = Bytes::from(&b"hello"[..]);
    assert_eq!(b.len(), 5);

    // Example 4
    let a = Bytes::from(&b"hello world"[..]);
    let b = a.slice(2..5);

    assert_eq!(&b[..], b"llo");
    
}
