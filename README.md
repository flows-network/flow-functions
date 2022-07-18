# Reactor WebAssmbly Functions

| Package Name | Inbound | Outbound | Description |
| ---- | ------- | -------- | ----------- |
| [star-messager](github/slack/star-messager/) | GitHub | Slack | Send a message to Slack when the GitHub repo gets every 10 stars |
| [issue-notifier](github/slack/issue-notifier/) | GitHub | Slack | Send a message to Slack when the GitHub issue is opened, edited, assigned  |
| [star-thanks-by-gmail](github/gmail/star-thanks-by-gmail/) | GitHub | Gmail | Send thank you message via Gmail when GitHub repo gets star |
| [star-thanks-by-sendgrid](github/sendgrid/star-thanks-by-sendgrid/) | GitHub | Sendgrid | Send thank you message via Sendgrid when GitHub repo gets star |
| [calculator](slack/slack/calculator/) | Slack | Slcak | Compute the expressions on the Slack |

## Build

* Install `wasm-pack` if you not

  ```shell
  rustup install wasm-pack
  ```

* Build packeage

  ```shell
  cargo build -p <package-name> --target wasm32-wasi --release # Build specified package
  cargo build --workspace --target wasm32-wasi --release       # Build all packages
  ```

## Usage

TODO
