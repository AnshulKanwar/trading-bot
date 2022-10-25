# trading-bot
A Trading Bot built using Rust

Currently it is built to only use the EMA crossover strategy

# Using

Get your [Binance](https://www.binance.com/en/binance-api) API key and API secret and put them in the `.env` file.

```
API_KEY="your_api_key"
API_SECRET="your_api_secret"
```

> **Warning**
> 
> Do not disclose your API keys. If the API keys were accidentally shared, please delete them immediately and create a new key.


Now run the bot using

```
$ trading-bot --help
A command line trading bot for binance

Usage: trading-bot [OPTIONS] --quantity <QUANTITY> --initial-side <INITIAL_SIDE> <SYMBOL>

Arguments:
  <SYMBOL>  symbol to use for trading. ex BTCUSDT

Options:
  -q, --quantity <QUANTITY>
          quantity of symbol to trade with
      --fast-ema <FAST_EMA>
          the value of span for fast ema [default: 10]
      --slow-ema <SLOW_EMA>
          the value of span for slow ema [default: 100]
  -i, --interval <INTERVAL>
          the interval or the time frame for the candle sticks [default: 30m]
  -s, --sleep-duration <SLEEP_DURATION>
          the duration the bot will wait before fetching for data [default: 10]
      --initial-side <INITIAL_SIDE>
          the initial position [possible values: buy, sell]
  -h, --help
          Print help information
  -V, --version
          Print version information
```

For example to run EMA crossover strategy with fast ema 20 and slow ema 100 on 10 BTCUSDT pair: 
```
$ cargo run -- --fast-ema 20 --slow-ema 100  --quantity 10 --last-move buy BTCUSDT
```

# TODO
- [ ] Add error handling
- [ ] Add tests 
- [ ] Remove the use of `initial_side`
- [x] Add logging
- [x] Add cli
