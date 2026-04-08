# hn-randfile-rust

一个使用rust写的随机文件生成器。

## 功能

- 批量生成随机名字、随机内容的文件。
- 支持同时指定多个输出目录。如果目录不存在，可选择是否创建。
- 默认生成文件数量为16，文件名长度为8，文件含1024个字节，后缀为txt。

## 安装与使用

### cargo

```bash
cargo install hn-randfile

# 指定多个目录
hn-randfile ./dir1 ./dir2 ./dir3

# 参数示例
hn-randfile ./files --number 10 --length 12 --size 2048 --suffix txt
```

### AUR

`paru -S hn-randfile-rust-bin
