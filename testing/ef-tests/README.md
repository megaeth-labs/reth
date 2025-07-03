# ef-tests

```sh
cd reth

git switch blockchain_test

cd testing/ef-tests 

git clone https://github.com/ethereum/tests.git ethereum-tests

mkdir -p ./ethereum-tests/BlockchainTests/ValidBlocks/bcMegaEthTest

cp ./blockchain_test.json ./ethereum-tests/BlockchainTests/ValidBlocks/bcMegaEthTest/

cargo test --package ef-tests --test tests --all-features -- megaeth_test::test_megaeth --exact --show-output
```
