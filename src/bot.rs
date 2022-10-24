use crate::indicators;
use crate::klines::{Kline, _Klines};
use crate::util;
use chrono::Utc;
use clap::ValueEnum;
use log::{info, debug};
use reqwest::{header, Url};
use std::{fmt, thread, time};

#[derive(Clone, ValueEnum)]
pub enum Side {
    Buy,
    Sell,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::Buy => write!(f, "BUY"),
            Side::Sell => write!(f, "SELL"),
        }
    }
}

pub struct Bot {
    client: reqwest::blocking::Client,
    api_secret: String,
    base_url: String,
}

impl Bot {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        let mut headers = header::HeaderMap::new();

        let mut api_key_header = header::HeaderValue::from_str(api_key).unwrap();
        api_key_header.set_sensitive(true);

        headers.insert("X-MBX-APIKEY", api_key_header);
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let client = reqwest::blocking::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Bot {
            client,
            api_secret: api_secret.to_owned(),
            base_url: String::from("https://api.binance.com"),
        }
    }

    pub fn run(
        &self,
        symbol: &str,
        quantity: f32,
        fast_ema: u32,
        slow_ema: u32,
        interval: &str,
        sleep_duration: &time::Duration,
        last_move: &Side,
    ) {
        let mut last_move = last_move.to_owned();
        loop {
            let klines = self.fetch_data(symbol, interval);
            let fast_ema_value = indicators::get_ema(&klines, fast_ema);
            let slow_ema_value = indicators::get_ema(&klines, slow_ema);

            debug!("fast ema ({}) = {}", fast_ema, fast_ema_value);
            debug!("slow ema ({}) = {}", slow_ema, slow_ema_value);

            match last_move {
                Side::Sell => {
                    if fast_ema_value > slow_ema_value {
                        info!("fast ema > slow ema");

                        self.order_market(symbol, &Side::Buy, quantity);

                        last_move = Side::Buy;
                    }
                }
                Side::Buy => {
                    if fast_ema_value < slow_ema_value {
                        info!("fast ema > slow ema");

                        self.order_market(symbol, &Side::Sell, quantity);

                        last_move = Side::Sell;
                    }
                }
            }

            thread::sleep(*sleep_duration);
        }
    }

    fn order_market(&self, symbol: &str, side: &Side, quantity: f32) {
        let url = self.base_url.to_owned() + "/api/v3/order";
        let url = Url::parse(&url).unwrap();

        let side = side.to_string();
        let quantity = quantity.to_string();
        let timestamp = Utc::now().timestamp_millis().to_string();

        let mut query = vec![
            ("symbol", symbol),
            ("side", &side),
            ("type", "MARKET"),
            ("quantity", &quantity),
            ("timestamp", &timestamp),
        ];

        let query_str: Vec<String> = query
            .iter()
            .map(|kv| kv.0.to_owned() + "=" + kv.1)
            .collect();

        let query_str = query_str.join("&");

        let signature = util::get_signature(&self.api_secret, &query_str);

        query.push(("signature", &signature));

        info!("order: {} {} {}", side, quantity, symbol);
        let request = self.client.post(url).query(&query).build().unwrap();
        println!("{}", request.url());

        let response = self.client.execute(request).unwrap();

        println!("{}", response.text().unwrap());
    }

    fn fetch_data(&self, symbol: &str, interval: &str) -> Vec<Kline> {
        let url = self.base_url.to_owned() + "/api/v3/klines";
        let url = Url::parse(&url).unwrap();

        debug!("fetching data");
        let response = self
            .client
            .get(url)
            .query(&[("symbol", symbol), ("interval", interval)])
            .send()
            .unwrap();

        let data = response.json::<_Klines>().unwrap();

        let mut klines: Vec<Kline> = data.0.iter().map(|kline| Kline::new(kline)).collect();

        // remove the open candle
        klines.pop();

        klines
    }
}
