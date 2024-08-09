# Neo Papyrus

## Dependency

```bash
sudo apt install libgtk-4-dev build-essential
```

## Problem

1. No IM module matching GTK_IM_MODULE=fcitx found

```bash
sudo apt install fcitx5
```

2. error: failed to run custom build command for gsk4-sys v0.9.0

检查一下gtk4.pc是否在PKG_CONFIG_PATH下：

```bash
echo $PGK_CONFIG_PATH
```

```bash
sudo find /usr -name gtk4.pc
```

```bash
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig
```

但是我想说的是，这里基本上跟上面的内容没有半毛钱关系。
主要是还是要检查你本机环境中的gtk4运行时的版本：

```bash
pkg-config --modversion gtk4
```

Output:

```
~/project/neo-papyrus git:[gtk-dev]
pkg-config --modversion gtk4
4.6.9
```

所以你要去检查一下你Cargo.toml中启用的gtk4的features版本，版本不能高于上述输出结果的4.6.9。