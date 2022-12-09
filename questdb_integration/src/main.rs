use questdb::{
    ingress::{Buffer, SenderBuilder},
    Result,
};

fn main() -> Result<()> {
    let mut sender = SenderBuilder::new("localhost", 9009).connect()?;
    let mut buffer = Buffer::new();

    buffer
        .table("btc")?
        .column_f64("PriceUSD", 30.0)?
        .at_now()?;

    sender.flush(&mut buffer)?;

    Ok(())
}
