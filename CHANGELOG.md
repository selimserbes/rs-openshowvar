# Changelog

## [0.1.0] - 2024-06-13

- Initial release: rs_openshowvar library is published.
- A Rust library that uses the OpenShowVar protocol to connect to Kuka robots and perform data read/write operations.

## [0.1.1] - 2024-06-13

- Removed redundant categories from `Cargo.toml`.

## [0.1.2] - 2024-06-13

- Corrected typos in the README.md file for better clarity and accuracy.

## [0.1.3] - 2024-06-13

- Fixed typos in the README.md file.

## [0.1.4] - 2024-06-13

- Erroneous sample code has been corrected.

## [0.1.5] - 2024-06-13

- Library definition errors have been fixed.
- Additional information notes were added to the README.md section.

## [0.1.6] - 2024-06-14

- Github repository name was changed from `rs_openshowvar` to `rs-openshowvar`.

## [0.1.7] - 2024-06-26

- Additional information notes were added to the README.md section.

## [0.1.8] - 2024-07-22

- Updated e2e tests and refined error handling.

## [1.0.0] - 2024-12-13

### Added

- `is_connected` method added to `OpenShowVar` for checking the connection status.

### Changed

- Library reached stable release with version `1.0.0`.

## [1.0.2] - 2025-05-22

### Added

- Added connection timeout in `connect()` method to limit the duration of establishing TCP connection.
- Added read/write timeouts in `send()` method to prevent indefinite blocking during TCP operations in the OpenShowVar library.
