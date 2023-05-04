# osu!purger
osu!purger is a small tool written in rust that looks through and deletes all files in  your osu! songs directory that are unnecessary for the game to function, as of now these files are: wav, mp4, jpg, png, mkv, jpeg, flv, ogg, wmv, mov and m4v.

# Table of Contents
- [Installation](https://github.com/jettosu/osu-purger#installation)
- [Usage](https://github.com/jettosu/osu-purger#usage)
- [To-do](https://github.com/jettosu/osu-purger#to-do)

## Installation
To install the tool simply download the newest release, extract the zip-file and open "osu_purger.exe".
 
## Usage
Once the program has opened, simply copy paste the path to your osu! song folder into the command line (in my case that is: D:\Games\osu!\Songs) and then press enter. The program will then go through the directory and remove all files deemed unimportant before outputting the total amount of files deleted together with their combined file size. Press enter once again to close the program after this message has displayed.

### Example
[![example.png](https://i.postimg.cc/bJqTNJSX/example.png)](https://postimg.cc/sB0ST3JK)

## To-do
- Allow user to select which filetypes they want removed from directory
    - Split into images, videoes and audio-files ✓
    - Potentially do so by defining the filetypes as different vectors and then pushing the wanted filetypes into a single one ✓
    - Find way to allow the user to select which file types they want to remove
- Properly setup project structure
    - Move code out of main into submodules ✓
    - Move code into the lib.rs module and out of main.rs, figure out how to do module-calls outside of main
- Implement security measures
    - Added requirement for additional confirmation from the user before starting deletion ✓
    - Have the program automatically identify the songs folder (if possible, else have it look for the default directory location and otherwise depend on user input)
- Potentially add GUI
    - Will be thought about further once other higher priority problems have been adressed
    - Look into [egui](https://github.com/emilk/egui)
- Refactor code
    - Decrease amount of times we iterate through the files vector, potentially by combining the indexing and filter funtion ✓
    - Make the code generally more readable and look into alternatives to the methods used in current
- Fix program occasionally deleting the song file due to inconsistent filetype standards
    - Potentially check file size and base it off that?
    - Add filter to not delete standard file names for songs ✓
- Make extension matching not case sensitive ✓
- Insert the user inputs into loops and 'continue' on invalid inputs ✓
    - Currently panics on any input other than "yes" or "no" in most cases, which is kinda stupid ✓

This is  the first thing I've written so I'm intentionally being rather vague with the goals as I honestly have no clue what those goals would involve.
