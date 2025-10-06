## Stellar Soroban Smart Contract Repository 

This repository contains soroban smart contracts for study purposes.It has abstracted devcontainer file for fast load.

- soroban-hello-world -> 
deployed on testnet  id:CCMVMAFN7MBRUBR2T4NBC22KW4GCCTVE5S6NOSLEM5AQEB4HIGZJS4LB

- soroban-increment -> 
deployed on testnet id:CCCOKAFQR5G5ABSWCJODK5OSFYENUWJSBH2HHQ2XP2EROHJIDTFYMCC6
added tests and functions 
    - decrement
    - reset
    - get_current_value

Added event firing functionality.For testing in each function invocation event vector is cleaned up 
so it need to be function call assert + event assert -> doesn't work as it explained in docs.

- soroban-customtype -> Its time add custom-type for increment contract.This way it can hold the incremented value and can reflect in events by incr-new_value.
deployed on testnet id:CCISHIKWHYKX3XRFEQWKIT72NZVP3EG546XR5FEBAMMPCD7KNQYARY5C
-


-- [Stellar explorer](https://stellar.expert/explorer/testnet) for checking ids.