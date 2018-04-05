%{
#include <stdio.h>
%}

%{
int lines=0;
%}

%%
^["/"{2}|"/"{3}].+   printf("%d, COMMENT ", ++lines);
[a-zA-Z][a-zA-Z0-9]* printf("%d, OTHER ", ++lines);
%%

//main( argc, argv )
//{
//    int argc;
//    char **argv;
//    {
//        ++argv, --argc;  /* passer le nom du programme */
//        if ( argc > 0 )
//            yyin = fopen( argv[0], "r" );
//        else
//            yyin = stdin;
//        yylex();
//    }
//    return 0;
//}
