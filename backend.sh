#!/bin/sh
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 OmaVLESS contributors
# QML calls this stable launcher; control.py owns runtime-sensitive lifecycle logic.
exec python3 "$(dirname "$0")/control.py" "$@"
