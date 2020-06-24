# ha-events

Home Assistant PC event webhook sender thing, for Ubuntu

 - When the app starts, it sends `ha-pc-on` webhook
 - When you ctrl+C (or send `SIGINT`), it sends `ha-pc-off` webhook
 - When screen locks, it sends `ha-pc-lock`
 - When screen unlocks, it sends `ha-pc-unlock`

Example use cases...

 - Turn on your office lights when your computer starts
 - Turn off your office lights when your computer shuts down
 - When screen lock/unlock happens, turn office lights off/on respectively

## Usage

Compile it, run it with `HA_WEBHOOK_BASE_URL` env var set, for example

```bash
HA_WEBHOOK_BASE_URL=http://your-ha-instance/api/webhook/ target/debug/ha-events
```

**NOTE** URL needs trailing slash

## Running on startup

The intended setup is that `ha-events` is set up as a "startup application":

More info: https://help.ubuntu.com/stable/ubuntu-help/startup-applications.html.en

 - **Name:** `ha-events`
 - **Description:** `Home Assistant webhook agent`
 - **Command:** `/bin/bash -c "sleep 5 && HA_WEBHOOK_BASE_URL=http://your-ha-instance/api/webhook/ /path/to/ha-events/target/debug/ha-events"`

I haven't found a way to set this up as a `systemd` service yet, since it
relies on `dbus-monitor` to watch for screen lock/unlock. Therefore it MUST be
run as your user in an environment where `dbus` is already launched; hence why
running as an Ubuntu "Startup Application".

The 5s delay is needed because it seems not everything is ready in time...
or something.

## Closing on logout/shutdown

Open your `~/.bash_logout`, and add:

```bash
killall -s SIGINT ha-events || true
```

## Only run with a specific monitor attached

If you only want to run `ha-events` when "docked" (e.g. by detecting if a specific monitor is attached), first, find
where the EDID file is, for example:

```bash
$ find /sys -name edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-HDMI-A-1/edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-HDMI-A-2/edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-VGA-1/edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-eDP-1/edid
/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-DP-1/edid
```

I want to detect a "Dell ABC123" monitor on DP-1, so I provide the `EDID_FILE_PATH` and `EXPECTED_MONITOR_ATTACHED`
environment variables to `ha-events` too:

```bash
HA_WEBHOOK_BASE_URL=http://your-ha-instance/api/webhook/ EDID_FILE_PATH=/sys/devices/pci0000:00/0000:00:02.0/drm/card0/card0-DP-1/edid EXPECTED_MONITOR_ATTACHED="DELL ABC123" /path/to/ha-events/target/debug/ha-events 
```
