fn main() {
    let a: u8 = 105;
    let b: u8 = 91;
    println!("a      = {:0>8b}", a);
    println!("b      = {:0>8b}", b);
    println!("a | b  = {:0>8b}", a | b);
    println!("a & b  = {:0>8b}", a & b);
    println!("a ^ b  = {:0>8b}", a ^ b);
    println!("!a     = {:0>8b}", !a);
    println!("a << 3 = {:0>8b}", a << 3);
    println!("a >> 3 = {:0>8b}", a >> 3);
}

/*
Example output:
--------------
a      = 01101001
b      = 01011011
a | b  = 01111011
a & b  = 01001001
a ^ b  = 00110010
!a     = 10010110
a << 3 = 01001000
a >> 3 = 00001101
*/
