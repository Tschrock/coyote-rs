# Coyote Connect

Coyote Connect is a set of libraries for controlling the Coyote estim power box from DG-Labs. It also contains a web app that provides a user-friendly interface for controlling the device and a REST API for developers to interact with.

Supported devices:
- Coyote v3

WIP:
- Pawprints
- Coyote v2

## App components
- `coyote-connect`: The main app
- `coyote-connect-web`: A web interface for controlling the device
- `coyote-connect-api`: A REST API for controlling the device
- `coyote-connect-osc`: An OSC server for controlling the device via OSC messages

## Library Components
- `coyote-connect-core`: The core library for controlling devices
- `coyote-connect-ble`: A library for controlling devices via Bluetooth Low Energy (BLE)
- `coyote-connect-websocket`: A library for controlling devices via WebSocket
- `coyote-connect-waveform`: A library for working with waveform files from the official app
