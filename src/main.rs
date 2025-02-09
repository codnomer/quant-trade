use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct OrderBook {
    bids: HashMap<Price, Limit>,
    asks: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }
    fn add_order(&mut self, price: f64, order: &Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price = Price::new(price);
                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        println!("Already have limits")
                    }
                    None => {
                        let mut limit = Limit::new(price);
                    }
                }
            }
            BidOrAsk::Ask => {}
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}
#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(size: f64, bid_or_ask: BidOrAsk) -> Order {
        Order { size, bid_or_ask }
    }
}

fn main() {
    let mut limit = Limit::new(50.0);
    let buy_order = Order::new(10.0, BidOrAsk::Bid);
    let orderbook = OrderBook::new();

    limit.add_order(buy_order);
    println!("Limit is : {:?}", limit);
    let price = Price::new(50.0);
    println!("Price is : {:?}", price);
}
