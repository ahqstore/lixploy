# Lixploy

**Lixploy** is a lightweight AppImage installer designed as a plugin for the [AHQ Store](https://github.com/ahqstore/client). It provides streamlined, silent installations of `.AppImage` packages on Linux without relying on traditional package managers.

---

## Features

- ✅ Installs `.AppImage` files with proper permissions
- ✅ Moves the executable to a persistent location
- ✅ Generates a `.desktop` entry
- ✅ Integrates with the AHQ Store's internal package registry

---

## Todo

- ⏲️ Support custom or extracted icons

---

## Supported Formats

| Format      | Description                 |
| ----------- | --------------------------- |
| `.AppImage` | Portable Linux applications |

Other formats (e.g., `.deb`, `.rpm`) are **intentionally** not supported.

---

## Usage

Lixploy is not a standalone package manager. It is invoked by the AHQ Store runtime during the installation of supported AppImage-based applications.
