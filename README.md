# Time App Tracker

## Description
This program allows to save the time spent on the pc and the desired applications in a database.

## Prerequisite
- cron (Verify that the service is installed and enabled)
- rust (install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- sqlite

## Installation
Installation of the application. Use the same command to update.
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
Usage: time_app_tracker [-v] [--state <state>] [--storage <storage>] [-s] [--add-notif <add-notif>] [--notif-time <notif-time>] [--del-notif <del-notif>] [--print-notif] [-u] [--add <add>] [--del <del>] [-q <query>] [--date <date>] [-n <number>] [-r]

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
  -q, --query       to retrieve data either for a day's info with [daydata] or
                    an application's info with [app-<name>]
  --date            select the date of the retrieved data, foramt : YYYY-mm-dd.
                    you can also use keywords such as yesterday, last_week or a
                    day of the week (monday…).
  -n, --number      select the number of day of the retrieved data
  -r, --reverse     inverts the result for an application
  -h, --help, help  display usage information
```

### Example of interface
#### The data of the day
##### Command
```
time_app_tracker -q daydata
```

##### Output
```
	Application time for 2025-03-04 :
pc : 7h56
alacritty : 4h33
nvim : 4h29
librewolf : 3h35
```

#### Last Tuesday's data
##### Command
```
time_app_tracker -q daydata --date tue
```

##### Output
```
    Application time for 2025-02-25 :
pc : 11h26
librewolf : 8h26
alacritty : 7h49
nvim : 4h38
```

#### Data for neovim over 3 days
##### Command
```
time_app_tracker -q app-nvim -n 3
```

##### Output
```
	Time for nvim :
2025-03-04 : 4h34
2025-03-03 : 6h22
2025-03-02 : 0m

	Stats of time for nvim :
Max : 6h22
Min : 0m
Sum : 10h56
Mean: 3h38
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

## Limitation
As the timer works with the cron service that is activated every minute, there is a margin of error of one minute each time an application is closed.

