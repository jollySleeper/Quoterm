# 🗨️ Quoterm
A CLI tool for greeting you and your terminal with beautiful quotes around the day.

![Quoterm](./screenshot.png)

## 📖 Table of Contents

- [✨ Features](#features)
- [🚀 Installation](#installation)
- [🛠️ Usage](#usage)
- [⚙️ Configuration](#configuration)
- [🗑️ Uninstall](#uninstall)
- [🐛 Bugs or Requests](#bugs-or-requests)
- [🤝 Contributing](#contributing)
- [📄 License](#license)
- [🙏 Acknowledgments](#acknowledgments)

## ✨ Features

- **Quotes**: Greets you with random quotes every time you open your terminal.
- **Reactive**: Formatting changes based on the width of your terminal.
- **Stay Motivated**: Get motivational & thought-provoking quotes.

## 🚀 Installation
> Please note that you should have [Rust](https://rust-lang.org) installed on your system.

- Clone this repo using
```bash
git clone http://github.com/jollySleeper/quoterm.git ~/quoterm && cd ~/quoterm
```

- Run these commands to install
```bash
cargo build --release
cargo install --path .
```

## 🛠️ Usage
> Please add `~/.cargo/bin/` to your PATH.

- Append the following snippet to your shell rc file:
```sh .bashrc/.zshrc
if type "quoterm" > /dev/null; then
    quoterm 
fi
```

## ⚙️ Configuration

Quoterm can be customized through a configuration file. The file is automatically created with default values when you first run the program.

### Location
- Linux/macOS: `~/.config/quoterm/config.json`
- Windows: `%APPDATA%\quoterm\config.json`

### Example Configuration
```json
{
  "colors": {
    "divider": "yellow",
    "quote": "blue",
    "author": "red"
  }
}
```

### Available Colors
Basic Colors:
- `"black"`
- `"red"`
- `"green"`
- `"yellow"`
- `"blue"`
- `"magenta"`
- `"cyan"`
- `"white"`

RGB Colors:
You can also specify custom RGB colors using an array of three numbers (red, green, blue) between 0 and 255:
```json
{
  "colors": {
    "divider": [255, 128, 0],  // Orange color
    "quote": "blue",           // Basic color
    "author": [100, 150, 200]  // Custom blue shade
  }
}
```

You can find an example configuration file in `config.example.json` in the repository.

## 🗑️ Uninstall

If you decide to uninstall, we're sorry to hear that `quoterm` didn't meet your expectations. We appreciate your feedback. 
To uninstall, run `rm ~/.cargo/bin/quoterm` from terminal and delete the Quoterm folder using `rm -r quoterm` command.

## 🐛 Bugs or Requests

If you encounter any problem(s) feel free to open an [issue](https://github.com/jollySleeper/quoterm/issues/new).
If you feel the project is missing a feature, please raise an [issue](https://github.com/jollySleeper/quoterm/issues/new) with `FeatureRequest` as heading.

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/YourFeature`).
3. Make your changes and commit them (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Open a pull request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/jollySleeper/quoterm/blob/main/LICENSE) file for details.

## 🙏 Acknowledgments

- This project takes inspiration from [Please](https://github.com/NayamAmarshe/please) and many other similar projects.
- The quotes displayed by this CLI tool is provided by the JSON File in [Please](https://github.com/NayamAmarshe/please).
