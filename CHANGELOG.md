# Changelog

## [Upcoming]

### Breading Changes

- Change visibility of `QueryBuilder::new()` and `QueryBuilder::to_string()` to avoid unintended use

### Improved

- Use builder pattern in `QueryBuilder` [[#1](https://github.com/kuy/jsonbox-rs/issues/1)]

## [0.2.0] 2019-09-28

### Breading Changes

- Make `Client::with_base_url()` a method instead of a constructor [[#3](https://github.com/kuy/jsonbox-rs/issues/3)]
- Change visibility of `Client::read_by_id()` and `Client::read_by_query()` (accidently exposed in past versions) [[#2](https://github.com/kuy/jsonbox-rs/issues/2)]

### Added

- Support bulk `CREATE` operation [[#9](https://github.com/kuy/jsonbox-rs/issues/9)]

## [0.1.2] 2019-09-28

### Added

- Add `QueryBuilder::and()` for better chaining [[#6](https://github.com/kuy/jsonbox-rs/issues/6)]

### Refactoring

- Remove `to_string` and `String` [[#10](https://github.com/kuy/jsonbox-rs/pull/10)] by [@rchaser53](https://github.com/rchaser53)

## [0.1.1] 2019-09-26

### Changed

- Add `updated_on` field to `Meta` struct [[#4](https://github.com/kuy/jsonbox-rs/issues/4)]

## [0.1.0] 2019-09-25

A version of nearly feature-complete.

### Added

- Support filter option (`q` param) for `READ` operation

## [0.0.3] 2019-09-24

### Breading Changes

- Now `Client::read()` returns `QueryBuilder` instance
- Use `QueryBuilder::all()` instead of `Client::read_all()`
- Use `QueryBuilder::id()` instead of `Client::read()`

### Added

- Support `sort`, `skip`, `limit` parameters in `READ` operation

### Changed

- Update `snafu 0.5.0` (I donn't know why I'm using `0.1.9`)

## [0.0.2] 2019-09-23

### Breading Changes

- Fix `Client::read_all()` to return result with meta data

## [0.0.1] 2019-09-22

### First Release
