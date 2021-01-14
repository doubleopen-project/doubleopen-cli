<!--
SPDX-FileCopyrightText: 2020 HH Partners

SPDX-License-Identifier: MIT
-->

# Double Open CLI

![Double Open logo](double_open_logo_huge.png)

[![REUSE status](https://api.reuse.software/badge/github.com/doubleopen-project/doubleopen-cli)](https://api.reuse.software/info/github.com/doubleopen-project/doubleopen-cli)

Analyze software projects for their bill of materials, get license and copyright data for them,
evaluate license compliance and build notice files.

## Goals

- Automate as much of the license compliance workflow as possible.
- Enable utilisation of other SPDX tooling by storing the data in spec compliant SPDX Documents.
- Store relevant data to enable vunlerability management based on the produced SPDX Documents.

## Pipeline

1. Analyze project with the Analyzer module to determine packages used in the projects and the
source files of each package. Save the data in an
[SPDX Document](https://spdx.github.io/spdx-spec/).
2. Populate the SPDX Document with license and copyright data from
[Fossology](https://github.com/fossology/fossology).
3. Evaluate the project's license compliance against a license policy.
4. Create a notice file for the project described in the SPDX Document.

## General architecture

- Independent modules for different parts of the pipeline:
  - SPDX:
    - Representation of the SPDX Document format with helper functions.
  - Analyze:
    - Analyzes the project and saves the data as SPDX.
  - Fossology:
    - Interact with Fossology's REST API.
  - Policy Engine:
    - Evaluate license data in an SPDX file against a license policy.
  - Notice:
    - Generate notice files from an SPDX file.
  - CLI:
    - CLI to use the modules.
- Store data in [SPDX Documents](https://spdx.github.io/spdx-spec/) in all steps of the pipeline.

## Project status

The project is under development, full functionality from start to finish is not yet to be
expected. We're currently working with a partner on implementing the pipeline as a proof of concept
for the [Yocto build system](https://www.yoctoproject.org/).

## License

MIT © HH Partners 2020-2021
