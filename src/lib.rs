#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serde_qs as qs;

pub mod binance;
pub mod binance_f;
pub mod huobi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
