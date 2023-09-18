# 🐶 Lyka

The Code Sharing CLI dedicated to my dog Lyka.

## What is Lyka?

Well, Lyka is a dog. My dog to be exact. She's a 5 year old Shih Tzu and she's the cutest thing ever. It's her birthday today (17th Sep), so I decided to make a CLI dedicated to her.

So, I have a PasteBin service called the NoobPaste which is hosted on my site. It has a public API which is used by this CLI to upload and download pastes. The CLI is written in rust. It is super fast and super easy to use.

You can find the Web Interface for the NoobPaste at [noobscience/code](https://noobscience.rocks/code)

## Installation

You can install the CLI using Cargo. Just run the following command:

```bash
cargo install lyka
```

You can also get it for windows from the [releases](https://github.com/newtoallofthis123/lyka_cli/releases) page.

## Usage

The CLI is super easy to use and you can use it in three ways:

1. Just run `lyka` and it will ask you for the paste. Once you enter the paste details in anyone one of the supported Text Editors, it will upload the paste and give you the link to the paste.

2. Run `lyka -f <path to file>` and it will upload the file along with asking for some details about the paste.

3. Run `lyka -d <paste id>` and it will download the paste with the given id to a file with the appropriate extension.

## Contributing

If you want to contribute to the project, you can do so by forking the repo and making a PR. I will be happy to review it and merge it if it's good.

## License

The project is licensed under the MIT License. You can read more about it in the [LICENSE](LICENSE) file.
