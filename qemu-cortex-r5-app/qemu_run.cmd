: # This is a special script which intermixes both sh
: # and cmd code. It is written this way because it is
: # set as the cargo runner and needs to work on Windows
: # and POSIX platforms. See https://stackoverflow.com/questions/17510688
: # for details.
:; ./qemu_run.sh $* ; exit
@ECHO OFF
call qemu_run.bat %*
