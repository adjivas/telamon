%option reentrant
%option noyywrap

%{
	#include <stdio.h>
	#include "token.h"

	int lines = 0;
%}

/* 6 Patterns § http://westes.github.io/flex/manual/Patterns.html#Patterns */
%%
^["/"{2}|"/"{3}].+   { ++lines; return COMMENT; }
[a-zA-Z][a-zA-Z0-9]* { ++lines; return OTHER; }

<<EOF>> return EOF;
%%
