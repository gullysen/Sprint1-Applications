## day 1

### pda

- global_config
- team_wallet
- tokenPda
- tokenMetaDataPda
- global_token_account

### functions

// configuration for contract
- config
    (
      token, 
      paymentToken, 
      amount, 
      supply, 
      decimal, 
      startPrice, 
      endPrice, 
      startDate, 
      endDate, 
      bonusReserve, // amount of tokens that will be used for bonus. Bonus will be paid until it's available
      bonusPercentage, // percent of bonus (with 2 decimals) which will be added to bought amount. I.e. 2500 = 25%
      bonusActivator   // percent of total ICO tokens that should be bought to activate bonus (with 2 decimals). I.e. 1000 = 10% 
    )         
- tokenLaunch ()
- createICO
    (
        token,
        payer,
        amount + bonusReserve
    )

- BuyToken
    (
        id,
        amount,
        buyer,
    )

- closeICO
    (

    )  

- withdraw()

-vesting 
    (
       to,
       amount,
       cliffFinish
       vestingPercentage,   //percentage of token will be unlocked every inverval
       vestingInterval      // vesting time(sec)
    ) // vesting tokens
