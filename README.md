# rust-tron-test
this is the example code to transfer in tron。 main codes is from [rust-tron](https://github.com/andelf/rust-tron).so, the detail you can refer to this rep.


# others
1. the account address in tron used base58 checked and have a 0x41	prefix .so, we need decode the adress with base58 and remove the firsr byte and the last byte. like this:`account[1..account.len()-4]`.this must handle by ourselve in trc20, because i used the ethernet lib [ethers](https://github.com/gakonst/ethers-rs)

2. the abi in the code is from usdt in eth.the base api is same as trc20 


