mod error;
mod utils;
mod blocks;
mod storage;
mod transactions;
mod wallets;

pub use blocks::*;
pub use storage::*;
pub use transactions::*;
pub use wallets::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
