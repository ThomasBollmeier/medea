from enum import Enum

class JsonType(Enum):
    NUMBER = "number"
    STRING = "string"
    BOOLEAN = "boolean"
    NULL = "null"
    ARRAY = "array"
    OBJECT = "object"


class JsonValue:
    def __init__(self, type_: JsonType):
        self._type = type_

    def get_type(self) -> JsonType:
        return self._type

    def accept(self, visitor: 'JsonVisitor'):
        raise NotImplementedError


class NumberValue(JsonValue):
    def __init__(self, value: float):
        super().__init__(JsonType.NUMBER)
        self._value = value

    def get_float_value(self) -> float:
        return self._value

    def accept(self, visitor: 'JsonVisitor'):
        return visitor.visit_number(self)


class StringValue(JsonValue):
    def __init__(self, value: str):
        super().__init__(JsonType.STRING)
        self._value = value

    def get_string_value(self) -> str:
        return self._value

    def accept(self, visitor: 'JsonVisitor'):
        return visitor.visit_string(self)


class BooleanValue(JsonValue):
    def __init__(self, value: bool):
        super().__init__(JsonType.BOOLEAN)
        self._value = value

    def get_bool_value(self) -> bool:
        return self._value

    def accept(self, visitor: 'JsonVisitor'):
        return visitor.visit_boolean(self)


class NullValue(JsonValue):
    def __init__(self):
        super().__init__(JsonType.NULL)

    def accept(self, visitor: 'JsonVisitor'):
        return visitor.visit_null(self)


class ArrayValue(JsonValue):
    def __init__(self, elements=None):
        super().__init__(JsonType.ARRAY)
        if elements is None:
            elements = []
        self._elements = elements

    def add_element(self, element: JsonValue):
        self._elements.append(element)

    def get_nth_element(self, n: int) -> JsonValue:
        return self._elements[n]

    def get_elements(self) -> list[JsonValue]:
        return self._elements

    def accept(self, visitor: 'JsonVisitor'):
        return visitor.visit_array(self)


class ObjectValue(JsonValue):
    def __init__(self, members: dict[str, JsonValue]=None):
        super().__init__(JsonType.OBJECT)
        if members is None:
            members = {}
        self._members = members

    def add_member(self, name: str, value: JsonValue):
        self._members[name] = value

    def get_member(self, name: str) -> JsonValue:
        return self._members.get(name)

    def get_members(self) -> list[str]:
        return list(self._members.keys())

    def accept(self, visitor: 'JsonVisitor'):
        return visitor.visit_object(self)


class JsonVisitor:
    def visit_number(self, value: NumberValue):
        pass

    def visit_string(self, value: StringValue):
        pass

    def visit_boolean(self, value: BooleanValue):
        pass

    def visit_null(self, value: NullValue):
        pass

    def visit_array(self, value: ArrayValue):
        pass

    def visit_object(self, value: ObjectValue):
        pass