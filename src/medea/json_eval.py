from medea.json_parser import JsonParser
from medea.values import *

def json_eval(json_string: str) -> JsonValue:
    ast = JsonParser().parse_text(json_string)
    return _json_eval_ast(ast)

def _json_eval_ast(ast) -> JsonValue:
    name = ast.name

    if name == "Number":
        return NumberValue(ast.value)
    elif name == "String":
        return StringValue(ast.value)
    elif name == "Boolean":
        return BooleanValue(ast.value)
    elif name == "Null":
        return NullValue()
    elif name == "JsonObject":
        obj = ObjectValue()
        for member in ast.children:
            key_node = member.children[0]
            value_node = member.children[1]
            key = key_node.value
            value = _json_eval_ast(value_node)
            obj.add_member(key, value)
        return obj
    elif name == "JsonArray":
        arr = ArrayValue()
        for element_node in ast.children:
            element = _json_eval_ast(element_node)
            arr.add_element(element)
        return arr

    raise RuntimeError("Unsupported AST node: " + name)

