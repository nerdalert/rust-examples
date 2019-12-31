use std::net::Ipv4Addr;

#[derive(Debug)]
struct BgpOpenMessage {
    version: u8,
    as_number: u16,
    holdtime: u16,
    bgp_id: Ipv4Addr,
    length: usize,
}

impl BgpOpenMessage {
    const VERSION: u8 = 4;
    const HOLDTIME: u16 = 90;

    fn new(id: Ipv4Addr, as_num: u16) -> BgpOpenMessage {
        BgpOpenMessage {
            version: BgpOpenMessage::VERSION,
            as_number: as_num,
            holdtime: BgpOpenMessage::HOLDTIME,
            bgp_id: id,
            length: 0,
        }
    }
    fn update(&mut self, id: Ipv4Addr) {
        self.bgp_id = id;
    }
}

fn main() {
    let bgp_id = Ipv4Addr::new(192, 168, 100, 1);
    let as_num = 65001;
    // Instantiate a new instance of BgpOpenMessage
    let mut bgp_msg = BgpOpenMessage::new(bgp_id, as_num);
    
    // Since we added #[derive(Debug)] to the struct,
    // we can print the instance with {:?}
    // and pretty print with {:#?}
    println!("BGP Open Message is -> {:#?}", bgp_msg);
    
    // Create a new ID
    let new_id = Ipv4Addr::new(192, 168, 200, 20);
    // Update the instance with a new ID
    bgp_msg.update(new_id);
    println!("BGP ID has been updated to -> {}", bgp_msg.bgp_id);
}

/*
Example output:
--------------
BGP Open Message is -> BgpOpenMessage {
    version: 4,
    as_number: 65001,
    holdtime: 90,
    bgp_id: 192.168.100.1,
    length: 0,
}
BGP ID has been updated to -> 192.168.200.20
*/
