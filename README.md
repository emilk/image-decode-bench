# Image Decode Benchmark

## Results on M1 MacBook

### Native
* `image`: 15.6 ms per jpeg
* `zune-jpeg` 10.3 ms per jpeg
* `zune-jpeg` `dev` branch: 8.2 ms per jpeg

### Web
* `image`: 35 ms per jpeg
* `zune-jpeg` 29 ms per jpeg
* `zune-jpeg` `dev` branch: 29 ms per jpeg

## Getting started

Start by clicking "Use this template" at https://github.com/emilk/image_decode_bench/ or follow [these instructions](https://docs.github.com/en/free-pro-team@latest/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template).

Change the name of the crate: Chose a good name for your project, and change the name to it in:
* `Cargo.toml`
    * Change the `package.name` from `image_decode_bench` to `your_crate`.
    * Change the `package.authors`
* `main.rs`
    * Change `image_decode_bench::TemplateApp` to `your_crate::TemplateApp`
* `index.html`
    * Change the `<title>Image Decode Benchmark</title>` to `<title>your_crate</title>`. optional.
* `assets/sw.js`
  * Change the `'./image_decode_bench.js'` to `./your_crate.js` (in `filesToCache` array)
  * Change the `'./image_decode_bench_bg.wasm'` to `./your_crate_bg.wasm` (in `filesToCache` array)


### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel fontconfig-devel`

### Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install Trunk with `cargo install --locked trunk`.
2. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
3. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Deploy
1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
3. Upload the `dist` directory to any of the numerous free hosting websites including [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).
4. we already provide a workflow that auto-deploys our app to GitHub pages if you enable it.
> To enable Github Pages, you need to go to Repository -> Settings -> Pages -> Source -> set to `gh-pages` branch and `/` (root).
>
> If `gh-pages` is not available in `Source`, just create and push a branch called `gh-pages` and it should be available.

You can test the template app at <https://emilk.github.io/image_decode_bench/>.
