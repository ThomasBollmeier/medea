from medea_prettify.json_parser import JsonParser
from alteraparser.ast_ import AstStrWriter


def test_json_object():

    code = """{ 
        "name": "John", 
        "age": 30, 
        "isStudent": true, 
        "courses": ["Math", "Science"], 
        "address": { "city": "New York", "zip": "10001" } 
    }"""

    run_code(code)


def test_json_array():

    code = """[
        { "name": "Alice", "age": 25 },
        { "name": "Bob", "age": 28 },
        { "name": "Charlie", "age": 22 }
    ]"""

    run_code(code)


def run_code(code: str):

    parser = JsonParser()
    parse_tree = parser.parse_text(code)

    assert parse_tree is not None

    writer = AstStrWriter()
    ast_str = writer.write_ast_to_str(parse_tree)
    print(ast_str)