/// replace creates a new String, and copies the data from
/// this string slice into it. While doing so, it attempts
/// to find matches of a pattern. If it finds any, it
/// replaces them with the replacement string slice.
fn main() {
    // option #1
    let host = "192.168.1.100";
    let option1 = str::replace(host, ".", "-");
    println!("{}", option1);
    // option #2
    let option2 = "docs.aws.amazon.com".replace(".", "-");
    println!("{}", option2);
}

/*
Example output:
--------------
192-168-1-100
docs-aws-amazon-com
*/
