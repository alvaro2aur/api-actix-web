# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-06-26
### Added
- A endpoint is implemented to provide data for the entity summary view. Currently, this endpoint returns test or simulated data.
- Database connection configuration. The schema changes dynamically at runtime depending on the `client.schema` attribute from the request body.
