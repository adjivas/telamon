(function() {var implementors = {};
implementors["telamon"] = [{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"telamon/ir/struct.Size.html\" title=\"struct telamon::ir::Size\">Size</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"telamon/codegen/struct.Size.html\" title=\"struct telamon::codegen::Size\">Size</a>&lt;'a&gt;",synthetic:false,types:["telamon::codegen::size::Size"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; for <a class=\"struct\" href=\"telamon/helper/tensor/struct.DimSize.html\" title=\"struct telamon::helper::tensor::DimSize\">DimSize</a>&lt;'a&gt;",synthetic:false,types:["telamon::helper::tensor::DimSize"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"telamon/helper/tensor/struct.DimSize.html\" title=\"struct telamon::helper::tensor::DimSize\">DimSize</a>&lt;'a&gt;",synthetic:false,types:["telamon::helper::tensor::DimSize"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"telamon/ir/struct.InstId.html\" title=\"struct telamon::ir::InstId\">InstId</a>&gt; for <a class=\"enum\" href=\"telamon/ir/enum.BBId.html\" title=\"enum telamon::ir::BBId\">BBId</a>",synthetic:false,types:["telamon::ir::basic_block::BBId"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"telamon/ir/struct.DimId.html\" title=\"struct telamon::ir::DimId\">DimId</a>&gt; for <a class=\"enum\" href=\"telamon/ir/enum.BBId.html\" title=\"enum telamon::ir::BBId\">BBId</a>",synthetic:false,types:["telamon::ir::basic_block::BBId"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"telamon/ir/enum.TypeError.html\" title=\"enum telamon::ir::TypeError\">TypeError</a>&gt; for <a class=\"enum\" href=\"telamon/ir/enum.Error.html\" title=\"enum telamon::ir::Error\">Error</a>",synthetic:false,types:["telamon::ir::error::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"telamon/ir/mem/struct.InternalId.html\" title=\"struct telamon::ir::mem::InternalId\">InternalId</a>&gt; for <a class=\"enum\" href=\"telamon/ir/mem/enum.MemId.html\" title=\"enum telamon::ir::mem::MemId\">MemId</a>",synthetic:false,types:["telamon::ir::mem::MemId"]},];
implementors["telamon_gen"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"telamon_gen/ast/enum.Statement.html\" title=\"enum telamon_gen::ast::Statement\">Statement</a>&gt; for <a class=\"enum\" href=\"telamon_gen/ast/enum.ChoiceDef.html\" title=\"enum telamon_gen::ast::ChoiceDef\">ChoiceDef</a>",synthetic:false,types:["telamon_gen::ast::choice::ChoiceDef"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"telamon_gen/lexer/struct.LexerPosition.html\" title=\"struct telamon_gen::lexer::LexerPosition\">LexerPosition</a>&gt; for <a class=\"struct\" href=\"telamon_gen/ast/struct.Position.html\" title=\"struct telamon_gen::ast::Position\">Position</a>",synthetic:false,types:["telamon_gen::lexer::ffi::Position"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Display.html\" title=\"struct std::path::Display\">Display</a>&lt;'a&gt;, ParseError&lt;<a class=\"struct\" href=\"telamon_gen/ast/struct.Position.html\" title=\"struct telamon_gen::ast::Position\">Position</a>, <a class=\"enum\" href=\"telamon_gen/lexer/enum.Token.html\" title=\"enum telamon_gen::lexer::Token\">Token</a>, <a class=\"enum\" href=\"telamon_gen/lexer/enum.LexicalError.html\" title=\"enum telamon_gen::lexer::LexicalError\">LexicalError</a>&gt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"telamon_gen/error/struct.ProcessError.html\" title=\"struct telamon_gen::error::ProcessError\">ProcessError</a>&lt;'a&gt;",synthetic:false,types:["telamon_gen::error::ProcessError"]},];
implementors["telamon_utils"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"telamon_utils/tfrecord/enum.ReadError.html\" title=\"enum telamon_utils::tfrecord::ReadError\">ReadError</a>",synthetic:false,types:["telamon_utils::tfrecord::ReadError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"telamon_utils/tfrecord/enum.WriteError.html\" title=\"enum telamon_utils::tfrecord::WriteError\">WriteError</a>",synthetic:false,types:["telamon_utils::tfrecord::WriteError"]},{text:"impl&lt;W&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/buffered/struct.IntoInnerError.html\" title=\"struct std::io::buffered::IntoInnerError\">IntoInnerError</a>&lt;W&gt;&gt; for <a class=\"enum\" href=\"telamon_utils/tfrecord/enum.WriteError.html\" title=\"enum telamon_utils::tfrecord::WriteError\">WriteError</a>",synthetic:false,types:["telamon_utils::tfrecord::WriteError"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"telamon_utils/struct.RcStr.html\" title=\"struct telamon_utils::RcStr\">RcStr</a>",synthetic:false,types:["telamon_utils::RcStr"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"telamon_utils/struct.RcStr.html\" title=\"struct telamon_utils::RcStr\">RcStr</a>",synthetic:false,types:["telamon_utils::RcStr"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
