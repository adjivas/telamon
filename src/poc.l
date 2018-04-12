%option reentrant
%option noyywrap

%{
	int lines = 0;

	enum token {
		COMMENT,
		OTHER,
	};
%}

/* 6 Patterns § http://westes.github.io/flex/manual/Patterns.html#Patterns */
%%
^["/"{2}|"/"{3}].+   { ++lines; return COMMENT; }
[a-zA-Z][a-zA-Z0-9]* { ++lines; return OTHER; }

<<EOF>> return EOF;
%%

int poc_yylex_init(yyscan_t *scanner)
{
	return yylex_init(scanner);
}

void ff(char *input /* , yyscan_t *scanner2 */ )
{
	yyscan_t scanner;
        // https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
	yylex_init(scanner);
//	poc_yylex_init(&scanner);

	// https://westes.github.io/flex/manual/Multiple-Input-Buffers.html
	write(1, input, 24);
	YY_BUFFER_STATE buffer_state = yy_scan_string(input, scanner);
	int code;
	do
        {
		code = yylex(scanner);
		printf("%d: (%d) -> \"%s\"\n", lines, code, yyget_text(scanner));
	}
        while (code != EOF);
	yy_delete_buffer(buffer_state, scanner);
	yylex_destroy(scanner);
}
