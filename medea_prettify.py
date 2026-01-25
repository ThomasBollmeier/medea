import sys
from argparse import ArgumentParser
import medea

def main():

    arg_parser = ArgumentParser()
    # Optional positional argument for file input
    arg_parser.add_argument(
        "file",
        help="The JSON file to read",
        nargs='?',
    )
    arg_parser.add_argument(
        "--indent",
        type=int,
        default=4,
        help="Number of spaces for indentation in the prettified JSON output.",
    )
    args = arg_parser.parse_args()

    if args.file:
        with open(args.file, "r") as f:
            json_string = f.read()
    else:
        json_string = sys.stdin.read()

    try:
        result = medea.json_prettify(json_string, args.indent)
        print(result)
    except SystemError:
        print("An error occurred while prettifying the JSON string.")


if __name__ == "__main__":
    main()
