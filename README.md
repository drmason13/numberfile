# NumberFile

A simple Application for numbering files to order them.

## Basic Usage

> These are planned features. They are checked once complete.

* [ ] prefix all files in the current directory with a number.
```sh
numberfile
```

* [ ] prefix all files at the given path, a folder or glob path.
```sh
# all markdown files in the book directory in the current folder
numberfile book/*.md
# all files in multiple folders (they will share a combined ordering, if you want *each* folder to start at 1 then run once on each folder)
numberfile books/*/
```


## Formatting

* [ ] use a custom prefix format where `{}` will be replaced with the number
```sh
numberfile -f 'Chapter {}'
```

* [ ] use a custom delimiter, the default is a hyphen `-`.
```sh
# use a space instead, i.e. "01 filename"
numberfile -d ' '
```

* [ ] You might want to use both options, as the delimiter is always included.
```sh
numberfile -f 'Chapter {}' -d ' '
```

## Ordering

NumberFile will order files according to their *modified time* by default.

* [ ] Instead order by their created time
```sh
numberfile --by-created
```

* [ ] Instead order by their current (unprefixed) filename
```sh
numberfile --by-name
```

## Padding

NumberFile uses an automatic number of digits padded with leading 0s (minimum of 2), e.g.
```
./
    a_file_which_is_oldest.txt
    myfile_new
```

becomes:
```
./
    01-myfile_new
    02-a_file_which_is_oldest.txt
```

* [ ] Override this behaviour and choose the number of digits using `-p` (padding)
```sh
# don't use any leading zeroes
numberfile -p 1

# pad to 3 places even if there are less than 100 files
numberfile -p 3
```

## Details

* [ ] Aims to be cross platform but I have not tested on a Mac.

* [ ] Tries to be smart and not "double" prefix filenames.

If it detects that a file already has a prefix that matches the format it will either
use those numbers and rename other files around it (the default) or
overwrite those prefixes to change the number to match the order of the files.