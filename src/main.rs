mod step1;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_step1 () {
    let mut balances = step1::BalanceModule::new();

    balances.set_balance(0,100);
    balances.set_balance(1,1000);

    assert!(balances.balance(0)==100);
    assert!(balances.balance(1)==1000);
    assert!(balances.balance(3)==0);

    assert!(balances.transfer(0, 1, 10).is_ok());

    assert!(balances.balance(0)==90);
    assert!(balances.balance(1)==1010);
}