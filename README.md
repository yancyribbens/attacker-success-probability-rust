# Attacker Success Probability

## Introduction
A Rust implementation to calculate the attack probability of an attacker ever reaching breakeven.  The original implementation by Satoshi can be found [here](https://bitcoin.org/bitcoin.pdf) on page 7.

## Paramaters
`z`: number of blocks created by the attacker (length of attack)

`q`: probability of the attacker finding the next block.  A high powered attacker will have a higer q value.

`P`: probability of attack success.

## Usage
Compile with rustc and execute the binary.

`rustc ./main.rs`

## Sample Output
```
q=0.1
z=0 P=1
z=1 P=0.20458727394278242
z=2 P=0.05097789283933862
z=3 P=0.013172241678896482
z=4 P=0.0034552434664853558
z=5 P=0.0009136821879277279
z=6 P=0.00024280274536281864
z=7 P=0.00006473531692709973
z=8 P=0.000017299804187072848
z=9 P=0.000004631163972502682
```
```
q=0.3
z=0 P=1
z=5 P=0.177352311360945
z=10 P=0.04166047996897908
z=15 P=0.010100762173110851
z=20 P=0.0024803981782356823
z=25 P=0.0006132283910057061
z=30 P=0.00015223394228240457
z=35 P=0.00003789576719277163
z=40 P=0.000009451722675110228
z=45 P=0.0000023607644378872776
z=50 P=0.0000005902953870559393
```
```
P < 0.001
z=5 q=0.1 P=0.0009136821879277279
z=8 q=0.15 P=0.0004222695605854794
z=11 q=0.2 P=0.0005601391091603293
z=15 q=0.25 P=0.000941061117069346
z=24 q=0.3 P=0.0008106495758241378
z=89 q=0.4 P=0.0009865729649406563
z=340 q=0.45 P=0.0009947214144901172
```
