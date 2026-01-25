from alteraparser.ast_ import Ast

class Number(Ast):
    def __init__(self, value: float):
        super().__init__("Number", value)

class String(Ast):
    def __init__(self, value: str):
        super().__init__("String", value)

class Boolean(Ast):
    def __init__(self, value: bool):
        super().__init__("Boolean", value)

class Null(Ast):
    def __init__(self):
        super().__init__("Null")

class JsonObject(Ast):
    def __init__(self, members: dict):
        super().__init__("JsonObject")
        for name, value in members.items():
            member = Ast("Member")
            member.add_child(Ast("Key", name))
            member.add_child(value)
            self.add_child(member)

class JsonArray(Ast):
    def __init__(self, elements: list):
        super().__init__("JsonArray")
        for element in elements:
            self.add_child(element)
