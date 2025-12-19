#!/bin/sh
set -e

dbus-daemon --system --fork
avahi-daemon --no-drop-root &

exec nginx -g "daemon off;"
