# Edenlog

Edenlog is a cli program that can assist you in finding specific lines in the game's log file.
It also has the ability to keep a running total of the bounty you've earned since the creation of that log file.
The output updates in real-time as more lines are written to the log file.

![image](http://0x0.st/H4aV.png)

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)

## Installation

Place the executable and config files in a folder together.

## Usage

The config file must be modified to have the path to your Gamelogs folder. The most recent log file in this folder is loaded automatically.
The search_words field is where you put the words you want to search for. Any line containing at least one of these words will be printed as output.
The stats field currently only supports stats for total bounty.

## Contributing

Issues and PRs will be reviewed. Feedback is welcome.
