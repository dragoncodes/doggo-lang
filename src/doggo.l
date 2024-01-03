%%
-?[0-9]+ "INTEGER"
\+ "ADD"
\* "MUL"
\/* "DIV"
\( "LPAR"
\) "RPAR"
let "LET"
\= "EQ"
[a-zA-Z][a-zA-Z0-9]* "ID"
[\t\n ]+ ;
