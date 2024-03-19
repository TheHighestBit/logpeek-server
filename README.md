# logpeek-server

Performant, web based log tail explorer written in Rust.

## Features

- **Web based frontend.** Allows for remote monitoring over the network.
- **Built-in dashboard.** Contains system information, error count per hour over the last 24h and more.
- **Performance first.** All log entries are kept in memory allowing for instant filtering. How many log entries are loaded at once is up to you.
- **Unlimited number of applications.** Each application is configured separately, allowing for processing of log entries with different structures.
- **Portability.** Distributed as a standalone binary. No installation required.
- **Configurability.** Both .toml and/or environment variable based configuration is supported.
- **Out-of-the-box compatibility with the [logpeek] crate.** When using `logpeek` as the logger in your Rust application, the generated logs will be compatible by default.

[logpeek]: https://crates.io/crates/logpeek
## Limitations
- All log entries must contain a timestamp, a log level, a module where the log entry originated from and the log message.
- Timestamps in the log entries must contain the offset in respect to UTC (should be the case in the vast majority of loggers).

## Getting Started

### Installing

You can find pre-built binaries for Linux, Windows and macOS on the [releases page].

[releases page]: https://github.com/TheHighestBit/logpeek-server/releases
### Building from source
Requirements (earlier versions will most likely work fine as well):
- Rust (tested with 1.76.0)
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
Let's say we have an application that is writing its log files to `logs/example-app/` and the log entries are structured as follows:

```
INFO at 2024-01-17T13:55:36.713756700+02:00 from example-app::main - This is an INFO message!
```

We can tell `logpeek-server` how to read these log files by modifying the `[[application]]` section in `config.toml`:
```toml
[[application]]
path = "logs/example-app/"
parser = '''^(?P<level>\S+) at (?P<timestamp>\S+) from (?P<module>\S+) - (?P<message>.+)$'''
timeformat = "iso8601"
```
Writing the regex parser was as simple as moving around the capture groups in the example found in `config.toml` and adding the separating words.
For more complex formats, you might need to modify the regex a little bit more.

For example, let's also modify the `config.toml` to only hold the last 1000 log entries in memory and enable authentication by setting the secret:
```toml
[main]
buffer_size = 1000
secret = "my_ultra_secret_password"
```

Now we can start the server by simply running the executable. It expects the `config.toml` to be in the same directory as the binary.

### Configuration

The recommended way of configuring logpeek-server is via `config.toml` (template can be found under `/backend` in the repo), which needs to be placed in the same directory as the binary.
Environment variables can be used as well, by prefixing them with `LOGPEEK_`.

Environment variables take precedence over the `config.toml` file.

### Features planned for v0.2.0 release
- [x] ~~**Make dashboard elements redirect to the log table with appropriate filters applied.** E.g. if the user sees that 5 hrs ago there was a spike of warnings, entering these filters into the log table manually
  is quite inconvenient. Instead, clicking on that column should do that automatically. The problem is that the current dashboard component doesn't expose any onClick events, so some hacking might be required.~~ 


- [ ] **Improve configuration.** Using a .toml file like this is not very user-friendly. There should be a page in the frontend that would allow the user to edit the config more conveniently.
The main reason this isn't implemented already is that the config library that is being used doesn't support writing to the config file and some refactoring might be necessary.


- [ ] **Add the option to filter by application.** This would be useful for both the dashboard and the table, implemented as a dropdown menu. Easiest way is to add a field to the log entry
struct and use that to filter.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
