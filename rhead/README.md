NAME
rhead – display first lines of a file

SYNOPSIS
rhead [-n count] [file ...]

DESCRIPTION
This filter displays the first count lines or bytes of each of the specified files,
or of the standard input if no files are specified. If count is omitted it defaults to 10.

     The following options are available:

     -n count
             Print count lines of each of the specified files.

     If more than a single file is specified, each file is preceded by a header consisting of the
     string "==> XXX <==" where XXX is the name of the file.
