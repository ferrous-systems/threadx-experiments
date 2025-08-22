: # This is a special script which intermixes both sh
: # and cmd code. It is written this way because it is
: # used in system() shell-outs directly in otherwise
: # portable code. See https://stackoverflow.com/questions/17510688
: # for details.
:; ./qemu_run.sh $* ; exit
@ECHO OFF
ECHO This is %COMSPEC%
call qemu_run.bat %*
