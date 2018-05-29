initSidebarItems({"enum":[["DirectoryOwnership",""]],"fn":[["byte_lit","Parse a string representing a byte literal into its final form. Similar to `char_lit`"],["byte_str_lit",""],["char_lit","Parse a string representing a character literal into its final form. Rather than just accepting/rejecting a given literal, unescapes it as well. Can take any slice prefixed by a character escape. Returns the character and the number of characters consumed."],["escape_default",""],["filemap_to_parser","Given a filemap and config, return a parser"],["filemap_to_stream","Given a filemap, produce a sequence of token-trees"],["float_lit",""],["integer_lit",""],["lit_token",""],["new_parser_from_file","Create a new parser, handling errors as appropriate if the file doesn't exist"],["new_parser_from_source_str",""],["new_parser_from_tts",""],["new_sub_parser_from_file","Given a session, a crate config, a path, and a span, add the file at the given path to the codemap, and return a parser. On an error, use the given span as the source of the problem."],["parse_crate_attrs_from_file",""],["parse_crate_attrs_from_source_str",""],["parse_crate_from_file",""],["parse_crate_from_source_str",""],["parse_expr_from_source_str",""],["parse_item_from_source_str","Parses an item."],["parse_meta_from_source_str",""],["parse_stmt_from_source_str",""],["parse_stream_from_source_str",""],["raw_str_lit","Parse a string representing a raw string literal into its final form. The only operation this does is convert embedded CRLF into a single LF."],["str_lit","Parse a string representing a string literal into its final form. Does unescaping."],["stream_to_parser","Given stream and the `ParseSess`, produce a parser"]],"mod":[["attr",""],["classify","Routines the parser uses to classify AST nodes"],["common","Common routines shared by parser mods"],["lexer",""],["obsolete","Support for parsing unsupported, old syntaxes, for the purpose of reporting errors. Parsing of these syntaxes is tested by compile-test/obsolete-syntax.rs."],["parser",""],["token",""]],"struct":[["Directory",""],["ParseSess","Info about a parsing session."]],"type":[["PResult",""]]});