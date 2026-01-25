from alteraparser.grammar import *
from medea.json_lexer_grammar import JsonLexerGrammar
from medea.json_ast import *


class JsonGrammar(Grammar):

    def __init__(self):
        super().__init__()

        lg = JsonLexerGrammar()

        @rule(self, "json", is_start_rule=True)
        def json(g):
            return g.value

        @ast_transformer(self, "json")
        def transform_json(ast):
            return ast.children[0]

        @rule(self, "value")
        def value(g):
            return choice(
                tok(lg.STRING),
                tok(lg.NUMBER),
                g.object,
                g.array,
                tok(lg.TRUE),
                tok(lg.FALSE),
                tok(lg.NULL),
            )

        @ast_transformer(self, "value")
        def transform_value(ast):
            child = ast.children[0]
            if child.name == "NUMBER":
                return Number(float(child.value))
            elif child.name == "STRING":
                return String(child.value[1:-1])  # Remove quotes
            elif child.name == "TRUE":
                return Boolean(True)
            elif child.name == "FALSE":
                return Boolean(False)
            elif child.name == "NULL":
                return Null()
            else:
                return child  # object or array

        @rule(self, "object")
        def object_(g):
            return seq(
                tok(lg.LBRACE),
                seq(tok(lg.STRING, "key"), tok(lg.COLON), g.value.set_id("value")),
                many(seq(tok(lg.COMMA), seq(tok(lg.STRING, "key"), tok(lg.COLON), g.value.set_id("value")))),
                tok(lg.RBRACE),
            )

        @ast_transformer(self, "object")
        def transform_object(ast):
            keys = [child.value[1:-1] for child in ast.get_children_by_id("key")]
            values = ast.get_children_by_id("value")
            members = dict(zip(keys, values))
            return JsonObject(members)

        @rule(self, "array")
        def array(g):
            return seq(
                tok(lg.LBRACKET),
                g.value.set_id("element"),
                many(seq(tok(lg.COMMA), g.value.set_id("element"))),
                tok(lg.RBRACKET),
            )

        @ast_transformer(self, "array")
        def transform_array(ast):
            return JsonArray(ast.get_children_by_id("element"))
