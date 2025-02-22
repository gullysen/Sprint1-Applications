## Application from yumecode

### Accounts

#### Launchpad

```
#[account]
pub struct Launchpad {
  pub vesting_implementation: Pubkey,
  pub owner: Pubkey,
  pub counter: u64,
  pub is_paused: bool,
}
```

#### Ico 
```
#[account]
pub struct Ico {
  pub token: Pubkey,
  pub payment_token: Pubkey,
  pub amount: u64,
  pub start_price: u64,
  pub end_price: u64,
  pub start_date: u64,
  pub end_date: u64,
  pub bonus_reserve: u64,
  pub bonus_percentage: u64,
  pub bonus_activator: u64,
  pub unlock_percentage: u64,
  pub cliff_period: u64,
  pub vesting_percentage: u64,
  pub vesting_interval: u64,
  pub ico_owner: Pubkey,
  pub is_closed: bool,
  pub total_sold: u64,
  pub total_received: u64,
}
```

### Instructions

#### initialize
- The initialize function sets up the launchpad with the owner and vesting implementation.
#### create_ico
- The create_ico function allows the owner to create a new ICO with specified parameters.
#### buy_token
- The buy_token function allows users to buy tokens from the ICO.
#### close_ico
- The close_ico function allows the ICO owner to close the ICO and refund unsold tokens.
#### set_pause_launchpad
- The set_pause_launchpad function allows the owner to pause or unpause the launchpad.
#### set_vesting_implementation
- The set_vesting_implementation function allows the owner to update the vesting implementation
#### rescue_tokens
- The rescue_tokens function allows the owner to rescue tokens from the contract.