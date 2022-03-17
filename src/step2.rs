use std::collections::HashMap;

type Account = u16;
type Balance = u32;

pub struct BalanceModule {
    balances : HashMap<Account,Balance>,
}

impl BalanceModule {
    pub fn new () -> Self {
        Self{
            balances: HashMap::new()
        }

    }

    pub fn set_balance(&mut self,who:Account,amount:Balance){
        self.balances.insert(who, amount);
    }

    pub fn balance(&self, index:Account) -> Balance {
        *self.balances.get(&index).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from:Account, to:Account, amount:Balance) -> Result<(),&'static str> {

        let from_balance = self.balances.get(&from).ok_or("user does not exist")?;
        let to_balance = self.balances.get(&to).unwrap_or(&0);

        let new_from_balance = from_balance.checked_sub(amount).ok_or("Not enough fund")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

