trait Bounceable {
    fn bounce(&self);
}

trait Inflatable {
    fn inflate(&self);
}

struct BasketBall {}

impl Bounceable for BasketBall {
    fn bounce(&self) {
        println!("Basket Ball is bouncing");
    }
}

impl Inflatable for BasketBall {
    fn inflate(&self) {
        println!("Basket Ball has been inflated");
    }
}

struct TennisBall {}

impl Bounceable for TennisBall {
    fn bounce(&self) {
        println!("Tennis Ball is bouncing");
    }
}

fn toss<T: Bounceable>(b: T) {
    b.bounce();
}

fn inflate<T: Inflatable>(i: T) {
    i.inflate();
}

fn main() {
    toss(BasketBall {});
    inflate(BasketBall {});

    toss(TennisBall {});
    //inflate(TennisBall{});
}
