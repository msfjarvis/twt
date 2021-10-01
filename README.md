# twitter-images

Fetches the last tweets of a given account, then prints original quality URLs for all image tweets. Useful for archiving image content from an art account without Twitter compression.

## Usage

There are unlikely to ever be published binaries for this, so this requires a Rust development environment set up locally.

```shell
git clone https://github.com/msfjarvis/twitter-images.git
cd twitter-images
cargo build --release
```

The tool is built to avoid interactive login and relies on the presence of a bunch of environment variables that require a Twitter developer account and a project created on the account to obtain.

- `CONSUMER_KEY` - The consumer API key for the project
- `CONSUMER_KEY_SECRET` - The consumer secret for the project
- `ACCESS_TOKEN` - Authentication access token for your user, for the project
- `ACCESS_TOKEN_SECRET` - Access secret for your user
- `TARGET_USERNAME` - The username of the account to fetch tweets from, such as `archillect`
- `MAX_AMOUNT` - Optional, specifies the maximum amount of tweets to check (default is 1024).
