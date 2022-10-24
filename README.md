# trading-bot
A Trading Bot built using Rust

Currently it is built to only use the EMA crossover strategy

# Using

```
$ trading-bot --help
Usage: trading-bot [OPTIONS] --quantity <QUANTITY> --last-move <LAST_MOVE> <SYMBOL>

Arguments:
  <SYMBOL>  

Options:
      --fast-ema <FAST_EMA>              [default: 10]
      --slow-ema <SLOW_EMA>              [default: 100]
  -i, --interval <INTERVAL>              [default: 30m]
  -s, --sleep-duration <SLEEP_DURATION>  [default: 600]
  -q, --quantity <QUANTITY>              
      --last-move <LAST_MOVE>            [possible values: buy, sell]
  -h, --help                             Print help information
  -V, --version                          Print version information
```

For example to run EMA crossover strategy with fast ema 20 and slow ema 100 on 10 BTCUSDT pair: 
```
$ cargo run -- --fast-ema 20 --slow-ema 100  --quantity 10 --last-move buy BTCUSDT
```

# TODO
- [ ] Add error handling
- [ ] Add logging
- [ ] log trades in a csv
- [ ] Add description for cli options
- [x] Add cli
