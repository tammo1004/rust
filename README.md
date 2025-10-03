# Rust(1.90.0) Programming Language
OS: Linux, MacOS

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```


## 확인, 업데이트, 삭제

```
$ rustc --version
$ rustup update
$ rustup self uninstall
```

```
$ rustc filename.rs
$ ./filename
```


## Cargo

```
$ cargo --version
```

```
$ cargo new new_cargo
$ cd new_cargo
```

```
$ cargo build
$ ./target/debug/new_cargo
```

```
$ cargo build --release
$ ./target/release/new_cargo
```

```
$ cargo run
```

```
$ cargo check
```


## VS Code, GitHub

F1 - Git: Clone - Clone from GitHub - URL 선택


Terminal

```
$ git config --local user.name "userID"
$ git config --local user.email "emailAddress"
```

```
$ git config --local user.name
$ git config --local user.email
```