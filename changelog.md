# Changelog

## [Unreleased]

### Added

### Changed

### Fixed

## [v2.0.0]

### Added

- Proper ndc-spec type `representation` for scalar types [#165](https://github.com/hasura/ndc-sqlserver/pull/165)

### Changed

### Fixed

- Properly stream introspection results [#155](https://github.com/hasura/ndc-sqlserver/pull/155)

## [v1.0.0]

### Fixed

- Added CA certs to container for TLS <https://github.com/hasura/ndc-sqlserver/pull/154>

## [v0.2.2]

### Changed

- ndc-spec version to v1.6.0

## [v0.2.1]

### Added

- Add subcommand `stored-procedures` to the `update` command to introspect stored procedures.

## [v0.2.0]

### Added

- Support for stored procedures

## [v0.1.2] - 2024-06-17

### Added

- Support for native mutations
- Support for reading the MSSQL connection string from an environment variable

## [v0.1.1] - 2024-05-15

### Added

- Support for CLI plugin for Arch Linux systems

## [v0.1.0] - 2024-04-30

- Initial release with support of ndc-spec v0.1.2 (beta)
  - Support for CLI plugin for Hasura v3 CLI, which allows the CLI to
    introspect the database on demand.
  - Query explain endpoint has been changed from `/explain` to `/query/explain`.
  - The default port was changed from 8100 to 8080.

<!-- end -->

[Unreleased]: https://github.com/hasura/ndc-sqlserver/compare/v0.2.3...HEAD
[v0.2.2]: https://github.com/hasura/ndc-sqlserver/releases/tag/v0.2.2
[v0.2.1]: https://github.com/hasura/ndc-sqlserver/releases/tag/v0.2.1
[v0.2.0]: https://github.com/hasura/ndc-sqlserver/releases/tag/v0.2.0
[v0.1.2]: https://github.com/hasura/ndc-sqlserver/releases/tag/v0.1.2
[v0.1.1]: https://github.com/hasura/ndc-sqlserver/releases/tag/v0.1.1
[v0.1.0]: https://github.com/hasura/ndc-sqlserver/releases/tag/v0.1.0
