# USING THIS IS A BAD IDEA IF YOU DON'T KNOW WHAT YOU'RE DOING!!!
------------
## osu!purger
osu!purger is a small tool written in rust that looks through and deletes all files in  your osu! songs directory that are unnecessary for the game to function, as of now these files are: wav, mp4, jpg, png, mkv, flv, jpeg, ogg, avi, wmv, mpg, mov, m4v, mpeg, 3gp, webm, webp, bmp, heif, svg and acc.

## Table of Contents
- [Installation](https://github.com/jettosu/osu-purger#installation)
- [Usage](https://github.com/jettosu/osu-purger#usage)
- [Goals](https://github.com/jettosu/osu-purger#goals)

## Installation
To install the tool simply download the newest release, extract the zip-file and open "osu_cleaner.exe".
 
## Usage
**I have implemented a total of 0 safety measures into this program, ensure that the file-path you enter is correct!! Otherwise you will potentially wipe every single file with the aforementioned extensions on your entire drive.**

Once the program has opened, simply copy paste the path to your osu! song folder into the command line (in my case that is: D:\Games\osu!\Songs) and then press enter. The program will then go through the directory and remove all files deemed unimportant before outputting the total amount of files deleted together with their combined file size. Press enter once again to close the program after this message has displayed.

## Goals
- Properly setup project structure
- Implement security measures
- Potentially add GUI
- Refactor code

This is  the first thing I've written so I'm intentionally being rather vague with the goals as I honestly have no clue what those goals would involve.
