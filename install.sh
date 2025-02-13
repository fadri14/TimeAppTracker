#! /usr/bin/env bash

if [[ $(whoami) != "root" ]]
then
    echo "Il faut être root"
    exit 1
fi

cp target/release/timetracker /usr/bin/timetracker
cp ressources/timetracker.service /etc/systemd/system
cp ressources/timetracker.timer/etc/systemd/system
systemctl --system daemon-reload
systemctl enable --now timetracker.timer
