#[macro_use]
extern crate error_chain;

pub mod binance_f;
pub mod huobi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
