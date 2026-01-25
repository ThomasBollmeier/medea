from medea.json_eval import json_eval
from medea.values import *


class JsonPrettier(JsonVisitor):
    def __init__(self, indent_size: int=4):
        super().__init__()
        self._indent_size = indent_size
        self._level = 0
        self._line_start = True
        self._output = ""

    def prettify(self, json_data: str) -> str:
        obj = json_eval(json_data)
        self._level = 0
        self._output = ""
        self._line_start = True

        obj.accept(self)

        return self._output

    def visit_number(self, value: NumberValue):
        self._write(str(value.get_float_value()))

    def visit_string(self, value: StringValue):
        self._write(f"\"{value.get_string_value()}\"")

    def visit_boolean(self, value: BooleanValue):
        self._write("true" if value.get_bool_value() else "false")

    def visit_null(self, value: NullValue):
        self._write("null")

    def visit_array(self, value: ArrayValue):
        self._writeln("[")
        self._indent()
        elements = value.get_elements()
        for i, element in enumerate(elements):
            element.accept(self)
            if i < len(elements) - 1:
                self._write(",")
            self._writeln()
        self._dedent()
        self._write("]")

    def visit_object(self, value: ObjectValue):
        self._writeln("{")
        self._indent()
        members = value.get_members()
        for i, key in enumerate(members):
            member_value = value.get_member(key)
            self._write(f"\"{key}\": ")
            member_value.accept(self)
            if i < len(members) - 1:
                self._write(",")
            self._writeln()
        self._dedent()
        self._write("}")

    def _indent(self):
        self._level += 1

    def _dedent(self):
        self._level -= 1

    def _write(self, text: str):
        if self._line_start:
            self._output += " " * (self._level * self._indent_size)
            self._line_start = False
        self._output += text

    def _writeln(self, text: str=""):
        self._write(text + "\n")
        self._line_start = True

