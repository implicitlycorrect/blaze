[![Crates.io](https://img.shields.io/crates/v/toy-arms?style=for-the-badge)](https://crates.io/crates/toy-arms)
[![Docs.rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/toy-arms)

<div align="center">

# :crossed_swords: toy-arms
<img src="https://user-images.githubusercontent.com/33578715/155048461-cb5cdd3f-6d59-4558-b3be-ce8b78144953.png" />

Huge thanks to my pal for this header [@suzuharuR](https://twitter.com/suzuharuR)

[Usage](#Usage)

</div>

# What's toy-arms?
This library is meant to be created to test rust's capability for game hacking. u know it's kinda far from normal programming, right? However, cuz I was too lazy and beginner this library is relatively high level, meaning it doesnt use either Nt or Zw API, and it's only capable of things that a user-mode application does. Dont get it something fancy, rather consider this as my training of rust, can't guarantee the quality of the code. Oh, and never ever mention about the bizzar project structure, even if it looks bloody cringy. My bad.

# :pushpin: Table of contents

- [:two_hearts: support me](#two_hearts-support-me)
- [:fire: Get started](#fire-get-started)
  - [step1](#step1)
  - [step2](#step2)
- [:herb: API info](#herb-api-info)

# :fire: Get started

But before actually test the example, I'll show you some preparation steps you're supposed to know.

## step1
Firstly, include `toy-arms` in your dependencies' table in `Cargo.toml`.

```toml
[dependencies]
toy-arms = {git = "https://github.com/pseuxide/toy-arms"}

# This annotation below is to tell the compiler to compile this into dll. MUST.
[lib]
crate-type = ["cdylib"]
```

## step2
You can specify target architecture in `.cargo/config.toml` as following:
```toml
[build]
target = "x86_64-pc-windows-gnu"
```

# Acknowledge
[hazedumper-rs](https://github.com/frk1/hazedumper-rs) - referenced as role model of pattern scan
