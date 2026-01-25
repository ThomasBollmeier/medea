from medea.json_eval import json_eval
from medea.values import ObjectValue, StringValue, NumberValue, BooleanValue, ArrayValue


def test_json_eval_object():

    code = '''{
        "name": "John",
        "age": 30,
        "isStudent": true
    }'''

    result = json_eval(code)
    assert isinstance(result, ObjectValue)
    assert isinstance(result.get_member("name"), StringValue)
    assert result.get_member("name").get_string_value() == "John"
    assert isinstance(result.get_member("age"), NumberValue)
    assert result.get_member("age").get_float_value() == 30
    assert isinstance(result.get_member("isStudent"), BooleanValue)
    assert result.get_member("isStudent").get_bool_value() is True

def test_json_eval_array():

    code = '''[
        { "name": "Alice", "age": 25 },
        { "name": "Bob", "age": 28 },
        { "name": "Charlie", "age": 22 }
    ]'''

    result = json_eval(code)
    assert isinstance(result, ArrayValue)
    assert len(result.get_elements()) == 3
    first_element = result.get_nth_element(0)
    assert isinstance(first_element, ObjectValue)
    assert first_element.get_member("name").get_string_value() == "Alice"
    assert first_element.get_member("age").get_float_value() == 25