# twt [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/) [![Built with Garnix](https://img.shields.io/endpoint?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fmsfjarvis%2Ftwt%3Fbranch%3Dmain)](https://garnix.io)

CLI tool to extract metadata from tweets

## Install

Check out the [website](https://msfjarvis.github.io/twt/)

## Setup

This tool requires Twitter consumer keys to function. Twitter is moving to [disallow free usage of the API](https://fxtwitter.com/twitterdev/status/1621026986784337922), so you will have to steal their own keys to work around this.

`twt` picks up keys from `$CONFIG_DIR/twt/config.toml` (see [here](https://docs.rs/dirs/latest/dirs/fn.config_dir.html) for your platform's interpretation of `$CONFIG_DIR`)

```toml
# config.toml
consumer_key = "totally_real_key"
consumer_key_secret = "h4xx0r"
```

## Usage

### Get image links

```
twt images --username archillect
```

### Get video links

```
twt videos --username imgur
```

### Set the maximum tweets to check

```
twt images --username archillect --max-amount 512
```

### Get all links

```
twt links --username AITA_online --host bit.ly
```

For more help run: `twt -h`.
