## run server and tests in watch mode
$ cargo install cargo-watch

cargo watch -q -c -w examples/ -x "run --example main"

cargo watch -q -c -w src/ -x "cargo run"

## TODO

- [ ] Get Order
    - [ ] search by txHash 
    - [ ] search by OrderId 
    - [ ] search by maker/taker 
- [ ] Get Orders
    - search by timeFrame
    - search by OrderAmouns
    - search by OrderPairs
- [ ] Time Series orders
- [ ] real Time orders
- [ ] user Stats
    - [ ] orders created
    - [ ] dominant chain
    - [ ] top 10
    - [ ] amounts traded
- [ ] orders by status
- [ ] seed stats
    - [ ] top stakers
    - [ ] top holders
    - [ ] traders