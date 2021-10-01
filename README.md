# twitter-images

Fetches the last tweets of a given account, then prints original quality URLs for all image tweets. Useful for archiving image content from an art account without Twitter compression.

## Usage

There are unlikely to ever be published binaries for this, so this requires a Rust development environment set up locally.

```shell
git clone https://github.com/msfjarvis/twitter-images.git
cd twitter-images
cargo build --release
```

The tool is built to avoid interactive login and relies on the presence of a bunch of environment variables/named arguments that require a Twitter developer account and a project created on the account to obtain.

- `CONSUMER_KEY` - The consumer API key for the project (`--consumer-secret`).
- `CONSUMER_KEY_SECRET` - The consumer secret for the project (`--consumer-key-secret`).
- `ACCESS_TOKEN` - Authentication access token for your user, for the project (`--access-token`).
- `ACCESS_TOKEN_SECRET` - Access secret for your user (`--access-token-secret`).

## Examples

- **Basic Usage**

    ```sh
    twitter-images archillect
    ```

- **Set the maximum tweets to check**

    ```sh
    twitter-images archillect --max-amount 512
    ```

- **Full Options**

    ```sh
    twitter-images archillect --access-token <access-token> --access-token-secret <access-token-secret> --consumer-key <consumer-key> --consumer-key-secret <consumer-key-secret>
    ```

For more help run: `twitter-images -h`.
