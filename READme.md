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
- soroban-errors -> This contracts shows how to  build custom errors for increment contract.Defining an Error
Contract errors are Rust u32 enums where every variant of the enum is assigned an integer. The #[contracterror] attribute is used to set the error up so it can be used in the return value of contract functions.

The enum has some constraints:

It must have the #[repr(u32)] attribute.
It must have the #[derive(Copy)] attribute.
Every variant must have an explicit integer value assigned.

----> If an error is returned from a function anything the function has done is rolled back. If ledger entries have been altered, or contract data stored, all those changes are reverted and will not be persisted.

deployed on testnet id:CBVPWMFO2NHWLFVSYBSXNJM36AAS2U3V6NCMDIFQ7QOMZZMLMJPQO6DI

- soroban-auth Build auth increment contract where only admin privileged account can use , added test suit
which check happy-fail pathes,  and checked the effect , 
----> Before deploying user should check admin user address if he has the signer keys !!! 

deployed on testnet id:CAQHSMS34QJ4FWIFQ22SSE4TSSRVRYOYX6Z5MAPWTLRLBWC3BXDANAWV


-- [Stellar explorer](https://stellar.expert/explorer/testnet) for checking ids.