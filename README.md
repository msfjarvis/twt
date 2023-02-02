# twt [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

CLI tool to extract metadata from tweets

## Install

### Install prebuilt binaries via shell script

```shell
# WARNING: this installer is experimental
curl --proto '=https' --tlsv1.2 -L -sSf https://github.com/msfjarvis/twt/releases/download/latest/installer.sh | sh
```

### Install prebuilt binaries via powershell script

```shell
# WARNING: this installer is experimental
irm 'https://github.com/msfjarvis/twt/releases/download/latest/installer.ps1' | iex
```

## Setup

This tool requires Twitter consumer keys to function. Twitter is moving to [disallow free usage of the API](https://fxtwitter.com/twitterdev/status/1621026986784337922) which is some hot bullshit, so you can pick up one of their own keys from [here](https://gist.github.com/shobotch/5160017) as a sincere fuck you to the new czar.

`twt` picks up keys from `$CONFIG_DIR/twt/config.toml` (see [here](https://docs.rs/dirs/latest/dirs/fn.config_dir.html) for your platform's interpretation of `$CONFIG_DIR`)

```toml
# config.toml
consumer_key = "totally_real_key"
consumer_key_secret = "h4xx0r"
```

## Usage

- **Get image links**

    ```sh
    twt images --username archillect
    ```

- **Get video links**

    ```sh
    twt videos --username imgur
    ```

- **Set the maximum tweets to check**

    ```sh
    twt images --username archillect --max-amount 512
    ```

- **Get all links**

    ```sh
    twt links --username AITA_online --host bit.ly
    ```

For more help run: `twt -h`.
