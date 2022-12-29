# _Astro_ x _Nannou_ Starter

```
# 0a. Rust
open https://www.rust-lang.org/tools/install

# 0b. Wasm-pack
open https://rustwasm.github.io/wasm-pack/installer/

# 1. RSW
cargo install rsw

# 2. Node dependencies
pnpm i

# 3a. Launch Astro + RSW (dev.)
pnpm dev:rsw

# 3b. Build all
pnpm build:rsw
```

## 🍾 Result

<img width="1824" alt="astro-nannou-starter-1" src="https://user-images.githubusercontent.com/603498/207852520-eb8c30b8-7a29-4b75-85cc-a00a41bfbb32.png">

## References

- https://github.com/tomoyanonymous/nannou-web-template (w. Webpack)
- https://github.com/rwasm/vite-plugin-rsw
- https://github.com/rwasm/rsw-rs
- https://nannou.cc
- https://github.com/rustwasm/wasm-pack

# Astro Starter Kit: Minimal

```
npm create astro@latest -- --template minimal
```

[![Open in StackBlitz](https://developer.stackblitz.com/img/open_in_stackblitz.svg)](https://stackblitz.com/github/withastro/astro/tree/latest/examples/minimal)
[![Open with CodeSandbox](https://assets.codesandbox.io/github/button-edit-lime.svg)](https://codesandbox.io/s/github/withastro/astro/tree/latest/examples/minimal)

> 🧑‍🚀 **Seasoned astronaut?** Delete this file. Have fun!

## 🚀 Project Structure

Inside of your Astro project, you'll see the following folders and files:

```
/
├── public/
├── src/
│   └── pages/
│       └── index.astro
└── package.json
```

Astro looks for `.astro` or `.md` files in the `src/pages/` directory. Each page is exposed as a route based on its file name.

There's nothing special about `src/components/`, but that's where we like to put any Astro/React/Vue/Svelte/Preact components.

Any static assets, like images, can be placed in the `public/` directory.

## 🧞 Commands

All commands are run from the root of the project, from a terminal:

| Command                | Action                                           |
| :--------------------- | :----------------------------------------------- |
| `npm install`          | Installs dependencies                            |
| `npm run dev`          | Starts local dev server at `localhost:3000`      |
| `npm run build`        | Build your production site to `./dist/`          |
| `npm run preview`      | Preview your build locally, before deploying     |
| `npm run astro ...`    | Run CLI commands like `astro add`, `astro check` |
| `npm run astro --help` | Get help using the Astro CLI                     |

## 👀 Want to learn more?

Feel free to check [our documentation](https://docs.astro.build) or jump into our [Discord server](https://astro.build/chat).
