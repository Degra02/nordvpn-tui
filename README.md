# nordvpn-tui

`nordvpn-tui` is a terminal-based user interface (TUI) tool built in Rust that provides a simple and interactive way to manage NordVPN connections. The tool allows users to browse, search, and select countries and cities for VPN connection directly from the terminal using keyboard navigation.

## Features

- **Scrollable country and city list**: Navigate through a list of all available countries and cities using arrow keys.
- **Search functionality**: Quickly search for countries and cities.
- **Vim-like key bindings**: Supports familiar keybindings such as `gg` to jump to the top and `G` to jump to the bottom of lists.
- **Keyboard-driven**: Navigate and select VPN servers entirely with your keyboard.
- **Connect with ease**: Instantly connect to the selected server using NordVPN's CLI.

## Installation

### Prerequisites

Before installing, make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [NordVPN CLI](https://nordvpn.com/download/linux/)
  
  Ensure you have access to the NordVPN command-line tool by verifying it is installed and accessible from the terminal:

  ```bash
  nordvpn --version
  ```

### Build from source

Clone the repository and build the project:

```bash
git clone https://github.com/Degra02/nordvpn-tui.git
cd nordvpn-tui
cargo build --release
```

To run the tool:

```bash
cargo run --release
```
or alternatively
```bash
./target/release/nordvpn-tui
```

## Usage

Once you start `nordvpn-tui`, you will be presented with a list of countries where NordVPN servers are available. You can navigate and connect using keyboard commands.

### Keyboard Shortcuts

| Key          | Action                                       |
|----------------------|----------------------------------------------|
| `K` or `Arrow Up`   | Move selection up                            |
| `J` or `Arrow Down` | Move selection down                          |
| `Enter`      | Select a country/city and connect to the VPN |
| `g g`        | Jump to the top of the list                  |
| `G`          | Jump to the bottom of the list               |
| `q`          | Quit the application                         |

## Example

Here is an example of the interface:

```
---------------------------------------
          nordvpn-tui
---------------------------------------
            Albania
            Algeria
            Andorra
            Argentina
            Armenia
            Australia
            Austria
            Azerbaijan
            Bahamas
            Bangladesh
            Belgium
...
Select <Enter> Down <Arrow-Down> Up <Arrow-Up> Quit <Q>
```

### Connect to a VPN

Once you've selected a country (or city if available), pressing `Enter` will automatically connect to the selected location using NordVPN.

## Contributing

Feel free to submit issues or pull requests to contribute to the development of `nordvpn-tui`. Contributions are welcome!

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -m 'Add some feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
