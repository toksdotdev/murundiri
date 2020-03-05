# 👮Murundiri

Murundiri is a blazing fast idepotency reverse proxy. It could also exist as an independent service (if you don't want to use it as a proxy.)

## Important

This is still a work in progress.

## Roadmap

- [ ] Implement Yaml config parser
- [ ] Add revers proxy handler
  - [ ] Implement proxy request interceptor
  - [ ] Validate request info based on idepotency rule
- [ ] Cache idempotency fields to redis
