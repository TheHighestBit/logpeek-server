# logpeek-server

Performant, web based log tail explorer written in Rust.

## Features

- **Web based frontend.** Allows for remote monitoring over the network.
- **Built-in dashboard.** Contains system information, error count per hour over the last 24h and more.
- **Performance first.** Log entries are kept in memory allowing for instant filtering. The exact amount is configurable.
- **Unlimited number of applications.** Each application is configured separately, allowing for processing of log entries with different structures.
- **Portability.** Distributed as a standalone binary. No installation required.
- **Regex based search.** Regex is supported for both module and message filters.
- **Out-of-the-box compatibility with the [logpeek] crate.** When using `logpeek` as the logger in your Rust application, the generated logs will be compatible by default.

[logpeek]: https://crates.io/crates/logpeek

## Screenshots
*Dashboard view*
![image](https://github.com/TheHighestBit/logpeek-server/assets/40504459/2ce50a13-f94d-4a48-91e4-aff944bcd025)

*Log table view*
![image](https://github.com/TheHighestBit/logpeek-server/assets/40504459/8b21c7c8-61ea-45a5-b557-be567618ceb6)

## Getting Started

### Installation

You can find pre-built binaries for Linux, Windows and macOS on the [releases page].

[releases page]: https://github.com/TheHighestBit/logpeek-server/releases
### Building from source
Requirements (earlier versions will most likely work fine as well):
- Rust (tested with 1.77.2)
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
`logpeek-server desired/path/to/config.toml`. Alternatively, you can also find the template under `/backend` in the repo.

For a concrete example, let's say we have an application that is writing its log files to `logs/example-app/` and the log entries are structured as follows:

```
INFO at 2024-01-17T13:55:36.713756700+02:00 from example-app::main - This is an INFO message!
```

We can tell `logpeek-server` how to read these log files by modifying the `[[application]]` section in `config.toml`:
```toml
[[application]]
path = "logs/example-app/"
parser = '''^(?P<level>\S+) at (?P<timestamp>\S+) from (?P<module>\S+) - (?P<message>.+)$'''
timeformat = "iso8601"
buffer_size = 1_000_000
```
Writing the regex parser was as simple as moving around the capture groups in the example found in `config.toml` and adding the separating words.
For more complex formats, you might need to modify the regex a little bit more or have the message field capture the entire line.

For another example, let's also enable authentication by setting the secret in `config.toml` and a better maximum number of login attempts:
```toml
[main]
secret = "my_ultra_secret_password"
max_login_attempts = 5
```

Now we can start the server by simply running the executable and specifying the path to the configuration file:
`logpeek-server path/to/config.toml`. The path can be omitted if the configuration file is named `config.toml` and is in the same directory as the binary.

### Configuration

The recommended way of configuring logpeek-server is via a TOML file.
Environment variables can be used as well, by prefixing them with `LOGPEEK_`.

Environment variables take precedence over the `config.toml` file.

### Features planned for v0.3.0 release
- [ ] **Improved configuration.** Using a .toml file like this is not very user-friendly. There should be a page in the frontend that would allow the user to edit the config more conveniently.
The main reason this isn't implemented already is that the config library that is being used doesn't support writing to the config file and some refactoring might be necessary.


- [ ] **Reduce binary size.**  The current application is written without much regard for it's size. A lot of bloat can be removed.


- [ ] **Add parsers for common log formats as options on the configuration page.** Altough regex is very powerful and flexible,
it can be a bit intimidating for some users. Having a dropdown with common log formats would make the setup process easier.
## License

This project is licensed under the MIT License - see the LICENSE file for details.
