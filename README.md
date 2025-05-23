## jmp

local, lightweight [duckduckgo bangs](https://duckduckgo.com/bangs) and [golinks](https://www.golinks.io/) for every browser.

<video src="https://github.com/user-attachments/assets/fbc4294e-bb07-457a-b83b-18ce42f2670a" width="100%" height="auto" muted></video>

## Features

Imagine, for example,
- `!yt rick` to search directly on YouTube.
- `!drs tokio` to search on docs.rs.
- `go/gh foo` to go to github.com/yourusername/foo.
- `go/w/cal` to go to your work calendar.

## Installation

```
cargo install --git https://github.com/ksdme/jmp
```

## How it works?

Well, you could build something like this as a browser extension. But, honestly, I know that to be an annoying
process, so, instead, jmp is a simple axum based web server that reads all configuration from a toml config
file.

## Setting it up

#### Hosting

You can either host it a server somewhere or just confiure your machine to start the server on startup. Running it
on your local machine instead has the benifit of it always being available, across all browsers. It also is better
in terms of security, especially because jmp doesn't yet have auth.

#### Configuring browsers

Once you have a running jmp server, you need to configure your browers to use it as the default search engine.
This might not be straight forward on some browsers, for example, Firefox. You can find more information [here](https://chatgpt.com/).

## Configuration

You can configure jmp using a toml config file. You can use [jmp.toml](./jmp.toml) as a reference or
you can stare at [conf.rs](./src/conf.rs) for the details.

### Todo

- Maybe make jump arguments more declarative.
- There a bug where placeholders in jumps are not interpolated if they aren't used.
