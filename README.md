# sunscreen-iai-bench

## Setups

Follow instructions at https://github.com/bheisler/iai

## Running

```sh
cargo bench
```

## Results

```
non_fhe_dot_product
  Instructions:             9261807
  L1 Accesses:             25028064
  L2 Accesses:               319902
  RAM Accesses:               17977
  Estimated Cycles:        27256769

fhe_dot_product
  Instructions:         12588501208
  L1 Accesses:          16599943107
  L2 Accesses:             80407435
  RAM Accesses:             6821929
  Estimated Cycles:     17240747797
```