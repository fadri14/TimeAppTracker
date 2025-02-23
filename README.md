# Time App Tracker

## Description
This program allows to save the time spent on the pc and the desired applications in a database.

## Prerequisite
- cron (Verify that the service is installed and enabled)
- rust (install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- sqlite

## Installation
Installation of the application.
```
cargo install time_app_tracker
```
Setting the timer.
```
bash -c 'TEMP_CRON_FILE=$(mktemp)
crontab -l > "$TEMP_CRON_FILE"
echo "* * * * * $HOME/.cargo/bin/time_app_tracker --update" >> "$TEMP_CRON_FILE"
crontab "$TEMP_CRON_FILE"
rm "$TEMP_CRON_FILE"'
```

## Usage

### Help
```
time_app_tracker --help
```

#### Output of help
```
Usage: time_app_tracker [-v] [--state <state>] [--storage <storage>] [-s] [--add-notif <add-notif>] [--notif-time <notif-time>] [--del-notif <del-notif>] [--print-notif] [-u] [--add <add>] [--del <del>] [-d] [-a <app>] [--date <date>] [-n <number>] [-r]

CLI to track usage times for pc and applications

Options:
  -v, --version     to get the current version number
  --state           switch between on and off state
  --storage         change the size of the storage
  -s, --settings    get the settings of this application
  --add-notif       enables notification mode for an application
  --notif-time      indicates the time in minutes before a notification is sent
  --del-notif       removes notification functionality for an application
  --print-notif     displays the list of notifications
  -u, --update      launch update
  --add             add a application
  --del             delete a application
  -d, --daydata     recover data from a day
  -a, --app         retrieve data from an application
  --date            select the date of the retrieved data, foramt : YYYY-mm-dd
  -n, --number      select the number of day of the retrieved data
  -r, --reverse     inverts the result for an application
  -h, --help, help  display usage information
```

### Example of interface
#### The data of the day
##### Command
```
time_app_tracker -d
```

##### Output
```
	Application time for 2025-02-23 :
pc : 13m
nvim : 7m
alacritty : 3m
```

#### Data for neovim over 3 days
##### Command
```
time_app_tracker -a nvim -n 3
```

##### Output
```
	Time for nvim :
2025-02-23 : 7m
2025-02-22 : 0m
2025-02-21 : 0m

	Stats of time for nvim :
Max : 7m
Min : 0m
Sum : 7m
Mean: 2m
```

#### Add a notification for the pc screen time after 3 hours and list the activated notifications
##### Command
```
time_app_tracker --add-notif pc --notif-time 180 --print-notif
```

##### Output
```
List of notifications :
pc => 3h
```

## Roadmap
- A TUI

## Limitation
As the timer works with the cron service that is activated every minute, there is a margin of error of one minute each time an application is used.

