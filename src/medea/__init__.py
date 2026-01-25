from medea.json_eval import json_eval

def json_prettify(json_data: str, indent: int=4) -> str:
    from medea.json_prettier import JsonPrettier
    prettier = JsonPrettier(indent)
    return prettier.prettify(json_data)