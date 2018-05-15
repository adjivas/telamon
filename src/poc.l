/* 19 Reentrant C Scanners § https://westes.github.io/flex/manual/Reentrant.html#Reentrant */
%option reentrant
/* https://westes.github.io/flex/manual/I-get-an-error-about-undefined-yywrap_0028_0029_002e.html#I-get-an-error-about-undefined-yywrap_0028_0029_002e */
%option noyywrap
/* number of the current line § https://westes.github.io/flex/manual/Options-Affecting-Scanner-Behavior.html#index-yylineno */
%option yylineno

%x WHITESPACE

%{
	typedef enum Token {
        VAR,
	} Token;

    typedef struct Pos {
        int line;
        int column;
    } Pos;

    typedef void * Data;

    typedef struct Span {
        Pos leg;
        Pos end;
        Data data;
    } Span;

    #define YY_EXTRA_TYPE Span
    #define YY_USER_ACTION { \
        yyextra.leg = yyextra.end; \
        yyextra.end.line += yylineno; \
        yyextra.end.column += yyleng; \
    }
%}

num [0-9]
alpha [a-zA-Z_]
alpha_num {alpha}|{num}
var \${alpha_num}+

whitespace [ \t\r\n]+
comment "//"[^/\n][^\n]*|"//"
blank {comment}|{whitespace}

c_comment_beg "/*"
c_comment_end "*/"

/* 6 Patterns § http://westes.github.io/flex/manual/Patterns.html#Patterns */
%%

{whitespace}          { BEGIN(WHITESPACE); }
<WHITESPACE>          {}

{var} { yytext++; return VAR; }

<<EOF>> { return EOF; };
%%

// https://westes.github.io/flex/manual/Index-of-Functions-and-Macros.html
int main(void)
{
    char *buffer = " $lll \n $ll";
    yyscan_t scanner = YY_NULL;

    // https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
    yylex_init(&scanner);

    // https://westes.github.io/flex/manual/Multiple-Input-Buffers.html
    YY_BUFFER_STATE buffer_state = yy_scan_bytes(buffer, strlen(buffer), scanner);

    int code;
    do {
        code = yylex(scanner);
        YY_EXTRA_TYPE extra = yyget_extra(scanner);
        printf("leg: [%d; %d], end: [%d; %d], code %d -> \"%s\"\n",
            extra.leg.column,
            extra.leg.line,
            extra.end.column,
            extra.end.line,
            code, yyget_text(scanner)
        );
    }
    while (code != EOF);

    yy_delete_buffer(buffer_state, scanner);
    yylex_destroy(scanner);
}
