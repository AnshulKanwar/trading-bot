use crate::klines::Kline;

pub fn get_ema(klines: &Vec<Kline>, span: u32) -> f32 {
    let k: f32 = 2.0 / (span + 1) as f32;
    let (first, rest) = klines.split_first().unwrap();

    let mut ema = first.close_price;

    for kline in rest {
        let price = kline.close_price;
        ema = k * price + (1.0 - k) * ema;
    }

    ema
}

// TODO: Write tests