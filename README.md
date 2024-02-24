# Witness-Encryption Implementation

一个简单的关于 Witness-Encryption 的实现

## 如何运行

整个 packedge 下共有三个包， 分别是
- `vss_gen`
- `encoder`
- `decoder`
分别代表 $\text{Vector Set Sum}$ 问题的生成， 加密模块和解密模块

### `vss_gen`

主目录下输入
```
cargo run --bin vss_gen
```
运行

输入为矩阵 $H$ 的行数 $d$ 和列数 $n$

### `encoder`

主目录下输入
```
cargo run --bin encoder -- <acc_gen>
```
运行

其中 `acc_gen` 参数有两种
- `-form`: 使用 Boolean Formula Encoding
- `-nss`: 使用 Nearly skew symmetric Matrices Encoding（暂未实现）

所以现在只能用
```
cargo run --bin encoder -- -form
```
来使用 `encoder`

运行时会要求输入 vss 输入文件的地址， 以及需要加密的信息 $b$ (1-bit)

### `decoder`

主目录下输入 
```
cargo run --bin decoder
```
运行

要求用户给出 `encoder.txt` 和 `witness.txt` 的地址， 最后解密出的信息会在标准输出中给出