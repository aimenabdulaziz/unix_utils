NAME
rcp – copy files

SYNOPSIS
rcp [-f | -i | -n] [-v] source_file target_file

DESCRIPTION
The rcp utility copies the contents of the source_file to the target_file.
If rcp detects an attempt to copy a file to itself, the copy will fail.

     The following options are available:

     -f    For each existing destination pathname, remove it and create a new file,
           without prompting for confirmation regardless of its permissions.
           (The -f option overrides any **previous** -i or -n options.)

     -i    Cause rcp to write a prompt to the standard error output before copying a file
           that would overwrite an existing file.  If the response from the standard input
           begins with the character 'y' or 'Y', the file copy is attempted.
           (The -i option overrides any **previous** -f or -n options.)
     -n    Do not overwrite an existing file.  (The -n option overrides any previous -f or -i options.)

     -v    Cause rcp to be verbose, showing files as they are copied.
