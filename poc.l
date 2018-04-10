%option reentrant
%option noyywrap

%{
	#include <stdio.h>
	#include "token.h"

	#define ECHO

	int lines = 0;
%}

%%
^["/"{2}|"/"{3}].+   { ++lines; return COMMENT; }
[a-zA-Z][a-zA-Z0-9]* { ++lines; return OTHER; }

<<EOF>> return EOF;
%%

// https://westes.github.io/flex/manual/Index-of-Functions-and-Macros.html
int main(void)
{
	char *input = "// ll \n ll";
	yyscan_t scanner;

        // https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
	yylex_init(&scanner);

	// https://westes.github.io/flex/manual/Multiple-Input-Buffers.html
	YY_BUFFER_STATE buffer_state = yy_scan_string(input, scanner);

	int lex_code;
	do
        {
		lex_code = yylex(scanner);
//		printf("ss %s\n", yyget_text(scanner));
//		printf("%d: (%d) \n", lines, lex_code);
	}
        while (lex_code != EOF);

	yy_delete_buffer(buffer_state, scanner);
	yylex_destroy(scanner);
}
