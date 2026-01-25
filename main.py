import medea

def main():
    json_string = '''{"name":"John Doe","age": 32,"isEmployed": false,"skills": ["Python", "C++", "JavaScript"],"address": {"street": "123 Main St","city": "Anytown","zip": "12345"}}'''

    try:
        result = medea.json_prettify(json_string, 8)
        print(result)
    except SystemError:
        print("An error occurred while prettifying the JSON string.")


if __name__ == "__main__":
    main()
