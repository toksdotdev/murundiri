# 👮Murundiri

Murundiri is a blazing fast and configurable idempotency reverse-proxy designed for scale.

## Features

- It can exist as either a `proxy` or `stand-alone` service.
- Allows supporte for integration with a redis cluster for caching.
- Support redis clusters (at least 3).
- Easily configuration via a `.yaml` file.
- Supports all HTTP verbs.
- Supports the following actions:
  - Service redirection (to any URI)
  - Returning custom JSON messages.
- Supports TTL for idempotency (i.e. time before idempotency expires)

## Important

The core implementation has been implemented, although, this isn't ready for production use yet.

## Roadmap

- [x] Implement murundiri config parser
- [x] Add reverse proxy
  - [x] Implement proxy request interceptor
  - [x] Validate request info based on idepotency rule
- [ ] Cache idempotency fields to redis
  - [x] Implement idempotency support.
  - [ ] Support 100% async for redis client ( i.e. both `normal` or `cluster` mode).
- [x] Support mounting of proxy on custom `ip` & `port`.
- [ ] Add proper Document the config file
- [ ] Add TLS support.
- [ ] Support infinity idempotency TTL.

## Contributing

If you find any issue, bug or missing feature, please kindly create an issue or submit a pull request, as it will go a long way in helping other [Rustaceans](https://www.rust-lang.org/community) grow.

## License

This repository is open-sourced software licensed under the [MIT](./LICENSE) license.
