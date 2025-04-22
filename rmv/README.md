NAME
rmv – move files

SYNOPSIS
rmv [-f | -i | -n] source target

DESCRIPTION
The rmv utility renames the file named by the source operand to the destination path named by the
target operand.

     The following options are available:

     -f      Do not prompt for confirmation before overwriting the destination path.
             If the target file already exists, it will be overwritten.
             (The -f option overrides any **previous** -i or -n options.)

     -i      Cause rmv to write a prompt to standard error before moving a file that would overwrite an existing file.
             If the response from the standard input begins with the character 'y' or 'Y', the move is attempted.
             (The -i option overrides any **previous** -f or -n options.)

     -n      Do not overwrite an existing file.  (The -n option overrides any **previous** -f or -i options.)
