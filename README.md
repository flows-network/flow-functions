# WasmHaiku Flow Functions

![badge](https://github.com/second-state/flow-functions/workflows/build/badge.svg?style=flat-square)

## Usage

| Package Name | Inbound | Outbound | Description |
| ---- | ------- | -------- | ----------- |
| [getting-started](github/slack/getting-started/) | GitHub | Slack | Send a message to Slack when new comments are added to GitHub issues |
| [star-messager](github/slack/star-messager/) | GitHub | Slack | Send a message to Slack when the GitHub repo gets every 10 stars |
| [issue-notifier](github/slack/issue-notifier/) | GitHub | Slack | Send a message to Slack when the GitHub issue is opened, edited, assigned  |
| [commit-syncer](github/slack/commit-syncer/) | GitHub | Slack | Send a message to Slack when the GitHub commits is pushed |
| [star-thanks-by-gmail](github/gmail/star-thanks-by-gmail/) | GitHub | Gmail | Send thank you message via Gmail when GitHub repo gets star |
| [star-thanks-by-sendgrid](github/sendgrid/star-thanks-by-sendgrid/) | GitHub | Sendgrid | Send thank you message via Sendgrid when GitHub repo gets star |
| [label-issue-discord](github/discord/issue-to-discord/) | GitHub | Discord | Send a message to Discord when the GitHub issue is opened, edited, assigned with a label |
| star-messenger | GitHub | [Twilio](github/twilio/star-messenger/) / [Slack](github/slack/star-messager/) | Send thank you message via Twilio(or Slack) when GitHub repo gets star |
| [calculator](slack/slack/calculator/) | Slack | Slcak | Compute the expressions on the Slack |
| [assign-notifier](github/notion/assign-notifier/) | GitHub | Notion | Create a task on Notion when the GitHub issue is assigned |
| [upload](slack/cloudinary/upload/) | Slack | Cloudinary | Upload a file from Slack to Cloudinary |
| [image-rotator](cloudinary/slack/image-rotator/) | Cloudinary | Slack | Returns the URL of the image rotated by 90 degrees when a file is uploaded to the Cloudinary |

## Build for Rust functions

* Install WASM target if you not

  ```shell
  rustup target add wasm32-wasi
  ```

* Build packeage

  ```shell
  cargo build -p <package-name> --target wasm32-wasi --release # Build specified package
  cargo build --workspace --target wasm32-wasi --release       # Build all packages
  ```


