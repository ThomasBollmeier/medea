from alteraparser.parser import TextParser
from medea.json_lexer_grammar import JsonLexerGrammar
from medea.json_grammar import JsonGrammar

class JsonParser(TextParser):

    def __init__(self):
        super().__init__(JsonGrammar(), JsonLexerGrammar())
