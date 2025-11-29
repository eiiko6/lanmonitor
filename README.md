# LanMonitor

Monitor your computer around your house because why not.

The daemon will only fully work on linux.

## Project Structure

**lanmonitor-daemon**, at the root of this repository, is the daemon meant to run on the host: the machine that sends its data.
It's written in pure rust and is pretty lightweight.  
It does use a LAN port so make sure your firewall allows it.

**lanmonitor**, in `./frontend/`, is the cross-platform application built with Tauri and Vuejs.

## How to use

- Execute **lanmonitor-daemon** on the machine you want to monitor. It should work perfectly on Linux, and partially on other systems.
- Then, execute **lanmonitor** on the machine you want to use as the monitor. This app can run on Android as well as on Linux and even Windows if you can compile it.

## How to compile

### lanmonitor-daemon

- `cargo build --release` at the root of this project

### lanmonitor

- `yarn tauri build` for Linux
- `yarn tauri android build --apk true` for Android. However you do need to setup `./frontend/src-tauri/gen/android/keystore.properties`. Refer to [Android Code Signing](https://v2.tauri.app/distribute/sign/android/).
