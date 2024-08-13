# logpeek-server

Logpeek-server is a log file explorer and monitoring tool written in Rust. It is designed to offer
near instant filtering and searching of log entries by keeping them in memory. The memory footprint is
completely configurable, allowing logpeek-server to be used on systems with limited and unlimited resources
alike.

It is to be noted that logpeek-server is mainly intended for use in home servers and is not suited
for production use with a high log throughput. As a general rule of thumb, 1 000 000 log entries take up
~100MB of RAM once they have been parsed.

## Features

- **Web based frontend.** Allows for remote monitoring over the network.
- **Built-in dashboard.** Contains system information, error count per hour over the last 24h and more.
- **Unlimited number of applications.** Each application is configured separately, allowing for processing of log
  entries with different structures.
- **Portability.** Distributed as a standalone binary. No installation required.
- **Regex based search.** Regex is supported for both module and message filters.

## Screenshots

*Dashboard view*
![image](https://github.com/user-attachments/assets/72a2d0f5-84eb-42b4-8097-123370b7d825)

*Log table view*
![image](https://github.com/user-attachments/assets/f553a661-54fa-442a-8c4e-1ee438c86a11)

## Getting Started

### Installation

You can find pre-built binaries for Linux and Windows on the [releases page].

[releases page]: https://github.com/TheHighestBit/logpeek-server/releases

### Building from source

Requirements (earlier versions will most likely work fine as well):

- Rust (tested with 1.80.0)
- Node.js (tested with 21.5.0)

```
git clone https://github.com/TheHighestBit/logpeek-server
cd logpeek-server/frontend
npm install
npm run build
cd ../backend
cargo build --release
```

The resulting binary can be found under `backend/target/release`.

### Example setup and usage

Let's first have logpeek-server generate the default configuration file for us to modify. For this,
simply run the application with the desired path to the configuration file as the argument:
`logpeek-server desired/path/to/config.toml`. Alternatively, you can also find the template under `/backend` in the
repo.

By default, logpeek-server is set up to monitor itself. However, for a more concrete example,
let's say we have an application that is writing its log files to `logs/example-app/` and the
log entries are structured as follows:

```
INFO at 2024-01-17T13:55:36.713756700+02:00 from example-app::main - This is an INFO message!
```

We can tell `logpeek-server` how to read these log files by adding a new `[[application]]` section
in `config.toml`:

```toml
[[application]]
path = "logs/example-app/"
name = "Example App"
parser = '''^(?P<level>\S+) at (?P<timestamp>\S+) from (?P<module>\S+) - (?P<message>.+)$'''
timeformat = "iso8601"
buffer_size = 1_000_000
```

Writing the regex parser was as simple as moving around the capture groups in the example found in `config.toml` and
adding the separating words.
For more complex formats, you might need to modify the regex a little bit more or have the message field capture the
entire line. We also set the buffer size to 1 000 000, meaning only the last 1 000 000
log entries of this application will be kept in memory at any given time.

Let's also enable authentication by setting the secret in `config.toml` and a better maximum number
of login attempts:

```toml
[main]
secret = "my_ultra_secret_password"
max_login_attempts = 5
```

Now we can start the server by simply running the executable and specifying the path to the configuration file:
`logpeek-server path/to/config.toml`. The path can be omitted if the configuration file is named `config.toml` and is in
the same directory as the binary.

### Configuration

The recommended way of configuring logpeek-server is via a TOML file.
Environment variables can be used as well, by prefixing them with `LOGPEEK_`.

For example, debug logs can be enabled with `LOGPEEK_main.logger.enable_debug=true`.

Environment variables take precedence over the config file.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
