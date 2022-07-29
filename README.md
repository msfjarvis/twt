# twt [![Check Rust code](https://github.com/msfjarvis/twitter-images/actions/workflows/test.yml/badge.svg)](https://github.com/msfjarvis/twitter-images/actions/workflows/test.yml)

CLI tool to extract metadata from tweets

## Usage

There are unlikely to ever be published binaries for this, so this requires a Rust development environment set up locally.

```shell
git clone https://github.com/msfjarvis/twitter-images.git
cd twitter-images
cargo build --release
```

The tool is built to avoid interactive login and relies on the presence of a bunch of environment variables at build-time that require a Twitter developer account and a project created on the account to obtain.

- `CONSUMER_KEY` - The consumer API key for the project.
- `CONSUMER_KEY_SECRET` - The consumer secret for the project.
- `ACCESS_TOKEN` - Authentication access token for your user, for the project.
- `ACCESS_TOKEN_SECRET` - Access secret for your user.

## Examples

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
