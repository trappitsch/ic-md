# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add additional configuration option to `CntSetup`: 
  - Counter is cleared by Z signal `CntClearedByZ`.
- Add `DeviceCfg` struct to configure the overall device prior to initialization. This will then be set by call to `init()`.
  - Input configuration for input pins `InputConfig`
  - Index signal configuration `IndexSignalConfig`
  - Touch probe pin configuration `TouchProbePinConfig`
  - Interface priority `InterfacePriority`
- Setters for all fields of `ic_md::CntSetup`.

### Removed

- Capability to create a new counter configuration with `CntSetup::new(...)`.

## [0.1.0](https://github.com/trappitsch/ic-md/releases/tag/v0.1.0) - 2025-08-27

### Added

- Start this changelog file
- Test for negative value issue ([#1](https://github.com/trappitsch/ic-md/pull/1))
- Document the current API.
- Add CI
- Add first, not-feature-complete version of the driver
- Add readme, license
