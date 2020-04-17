# Web Assembly Demo (Rust & Node)

This source code was the demo for the talk titled, "[Web Assembly: Is it really next generation](https://bit.ly/recording-wasm-next-generation)"?

Listed below are the links to the talk materials.

- 📹[Video Recording](https://bit.ly/recording-wasm-next-generation)
- 📺[Presentation Slides](https://bit.ly/slide-wasm-next-generation)

## 🛠 Requirements

To successfully setup this project, ensure you have installed the following:

- [NodeJS](https://nodejs.org/)
- [Rust Compiler](https://http://rust-lang.org)
- [wasm-pack CLI](https://github.com/rustwasm/wasm-pack) (this requirs the Rust Compiler mentioned above).

## 💻 Code Setup

With your termimal:

- `cd` into the [rust](./rust) directory, and run the following commands:

```bash
wasm-pack build
```

- `cd` into the [javascript](./javascript) directory, and run the following commands:

```bash
npm install
npm run start
```

This should serve your app on `localhost:8080` or the port might be different if an existing app is making use of the current port.

- Open the URL on your browser & enjoy 🚀.

## 👩🏽‍🏫 Understanding the Code

The source is broken into 2 sections. The [javascript](./javascript) and the [rust](./rust) directory.

The [rust](./rust) is responsible for parsing the stringified JSON, and gets compiled
to `.wasm` using [wasm-pack](https://github.com/rustwasm/wasm-pack).

Once the `.wasm` file has been generated, it is [pre-registered as a package](./javascript/package.json) in the
`javascript` app.

The Javascript app will always see the wasm module as an npm package, and allow our `javascript` app to execute it.

## 💪🏽 Contribution

Do you feel something isn't right or you have an cool feature in mind, kindly open an issue or create a pull request.

I'll be glad to review it and possibly merge it.

## 🔑 License

[MIT](./LICENSE) © Tochukwu Nkemdilim
