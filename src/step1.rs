use std::collections::HashMap;

pub struct BalanceModule {
    balances : HashMap<u32,u32>,
}

impl BalanceModule {
    pub fn new () -> Self {
        Self{
            balances: HashMap::new()
        }

    }

    pub fn set_balance(&mut self,who:u32,amount:u32){
        self.balances.insert(who, amount);
    }

    pub fn balance(&self, index:u32) -> u32 {
        *self.balances.get(&index).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from:u32, to:u32, amount:u32) -> Result<(),&'static str> {

        let from_balance = self.balances.get(&from).ok_or("user does not exist")?;
        let to_balance = self.balances.get(&to).unwrap_or(&0);

        let new_from_balance = from_balance.checked_sub(amount).ok_or("Not enough fund")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Not enough fund")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

