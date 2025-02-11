mod matching;
use matching::orderbook::{BidOrAsk, Order, OrderBook};
fn main() {
    let buy_order = Order::new(5.0, BidOrAsk::Bid);
    let buy_order_second = Order::new(5.0, BidOrAsk::Bid);
    //let sell_order = Order::new(5.0, BidOrAsk::Ask);
    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order);
    orderbook.add_order(4.4, buy_order_second);

    let sell_order = Order::new(4.0, BidOrAsk::Ask);
    Order::new(4.0, BidOrAsk::Ask);
    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, sell_order);
    println!("Orderbook sell condition: {:?}", orderbook);
    println!("OrderBook  is : {:?}", orderbook);
}
