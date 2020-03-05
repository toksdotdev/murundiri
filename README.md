# 👮Murundiri

Murundiri is a blazing fast and stand-alone idempotency enforcing proxy designed for scale.

## Features

- It can exist as either a `proxy` or `stand-alone` service.
- Allows supporte for integration with a redis cluster for caching.

## Important

This is still a work in progress.

## Roadmap

- [x] Implement murundiri config parser
- [ ] Add reverse proxy
  - [ ] Implement proxy request interceptor
  - [ ] Validate request info based on idepotency rule
- [ ] Cache idempotency fields to redis
