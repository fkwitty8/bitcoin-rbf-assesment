
1. `for creating folder and files simultaneousily`
substitute with your file path and make sure you are using window.. command change with linux, macOS
```
New-Item -ItemType File -Path "src/tests/infrastructure/bitcoin_rpc/adapters/wallet_adapter_test.rs" -Force
```

2. ` running test`: you replace specific test to be run
```
cargo test --test wallet_adapter_test
cargo test --test blockchain_adapter_test

```
2.a. ` run all test for a give layer`: runn infratructure tests
```
cargo test infrastructure::
```

2.b. ` run all test in in the entire test/
```
cargo test
```
