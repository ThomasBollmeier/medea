from alteraparser.lexer_grammar import LexerGrammar

class JsonLexerGrammar(LexerGrammar):

    def __init__(self):
        super().__init__()

        self.add_rule("WHITESPACE", r'\s+', ignore=True)  # Ignore whitespace
        self.add_rule("NUMBER", r'-?\d+(\.\d+)?([eE][+-]?\d+)?')
        self.add_rule("STRING", r'"(\\.|[^"\\])*"')
        self.add_rule("TRUE", r'true')
        self.add_rule("FALSE", r'false')
        self.add_rule("NULL", r'null')
        self.add_rule("LBRACE", r'\{')
        self.add_rule("RBRACE", r'\}')
        self.add_rule("LBRACKET", r'\[')
        self.add_rule("RBRACKET", r'\]')
        self.add_rule("COMMA", r',')
        self.add_rule("COLON", r':')
