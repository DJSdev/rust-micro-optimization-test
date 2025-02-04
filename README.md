# Exploration

Just exploring the performance impact (nanoseconds vs microseconds) when changing a value on a struct using a reference vs value

## Debug mode results

```sh
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\test3-rs.exe`
2.3µs to update name to Bob
100ns to update name to Tom
```

## Release mode results

```sh
$ cargo run --release
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target\release\test3-rs.exe`
300ns to update name to Bob
100ns to update name to Tom
```