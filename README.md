# USING THIS IS A BAD IDEA IF YOU DON'T KNOW WHAT YOU'RE DOING DUE TO LACK OF SECURITY FEATURES!!!
------------
## osu!purger
osu!purger is a small tool written in rust that looks through and deletes all files in  your osu! songs directory that are unnecessary for the game to function, as of now these files are: wav, mp4, jpg, png, mkv, flv, jpeg, ogg, avi, wmv, mpg, mov, m4v, mpeg, 3gp, webm, webp, bmp, heif, svg and acc.

## Table of Contents
- [Installation](https://github.com/jettosu/osu-purger#installation)
- [Usage](https://github.com/jettosu/osu-purger#usage)
- [Goals](https://github.com/jettosu/osu-purger#goals)

## Installation
To install the tool simply download the newest release, extract the zip-file and open "osu_purger.exe".
 
## Usage
Once the program has opened, simply copy paste the path to your osu! song folder into the command line (in my case that is: D:\Games\osu!\Songs) and then press enter. The program will then go through the directory and remove all files deemed unimportant before outputting the total amount of files deleted together with their combined file size. Press enter once again to close the program after this message has displayed.

## Goals
- Properly setup project structure
    - Refactored most of the code into the library module, will look into organizing it further / more properly
- Implement security measures
    - Added requirement for additional confirmation from the user before starting deletion
    - Look into having the program automatically identify the songs folder (if possible)
- Potentially add GUI
    - Just a consideration, will look into it once the other issues have been addressed as I believe those are higher priority
- Refactor code
    - Have to rewrite it a bunch, will likely happen when I've organized the project files properly

This is  the first thing I've written so I'm intentionally being rather vague with the goals as I honestly have no clue what those goals would involve.
