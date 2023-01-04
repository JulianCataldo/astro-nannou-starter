# _Astro_ x _Nannou_ Starter

https://user-images.githubusercontent.com/603498/210013817-1866e0bb-427f-444b-92cb-172f94d26cb0.mov

<div align="center">

🕹  [**Try it** online!](https://juliancataldo.github.io/astro-nannou-starter/)

</div>

---

```
# 0a. Rust language tools
open https://www.rust-lang.org/tools/install

# 0b. wasm-pack
open https://rustwasm.github.io/wasm-pack/installer

# 1. rsw = rs(rust) → w(wasm)
cargo install rsw

# 2. Install NodeJS dependencies
pnpm i

# 3a. Launch AstroJS + rsw (dev.)
pnpm dev:rsw

# 3b. Build all (prod.)
pnpm build:rsw
```

See also the full **GitHub workflow**: [./.github/workflows/release.yml](./.github/workflows/release.yml).  
Can be tested locally with [Act, a GitHub workflow runner](https://github.com/nektos/act).

## 🍾  Result

<img width="1824" alt="astro-nannou-starter-1" src="https://user-images.githubusercontent.com/603498/207852520-eb8c30b8-7a29-4b75-85cc-a00a41bfbb32.png">

## References

- https://github.com/tomoyanonymous/nannou-web-template (w. Webpack)
- https://github.com/rwasm/vite-plugin-rsw
- https://github.com/rwasm/rsw-rs
- https://nannou.cc
- https://github.com/rustwasm/wasm-pack

## Would be nice to have

- Dynamic sketch switcher
- Multiple sketch per page (without iframe)

---

🔗  [JulianCataldo.com](https://www.juliancataldo.com/)
