#!/usr/bin/env python3

import argparse
import json
import jsonschema
from pathlib import Path

def validate_json(example_file: str) -> None:
    # Load the schema
    schema_path = Path(__file__).parent / 'spell-template.jsonschema'
    with open(schema_path) as f:
        schema = json.load(f)

    # Load the example JSON (from the provided file or default)
    example_path = Path(__file__).parent / example_file
    with open(example_path) as f:
        example = json.load(f)

    try:
        # Validate the example JSON against the schema
        jsonschema.validate(instance=example, schema=schema)
        print(f"✅ Validation successful! {example_file} is valid according to the schema.")
    except jsonschema.exceptions.ValidationError as e:
        print(f"❌ Validation failed for {example_file}!")
        print(f"Error: {e.message}")
        print(f"Path: {' -> '.join(str(p) for p in e.path)}")
        print(f"Schema path: {' -> '.join(str(p) for p in e.schema_path)}")
    except Exception as e:
        print(f"❌ An unexpected error occurred: {str(e)}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Validate a JSON example (e.g. Carnage) against the spell schema.")
    parser.add_argument("--file", type=str, default="spell-example-2.json", help="JSON example file (default: spell-example-2.json)")
    args = parser.parse_args()
    validate_json(args.file)
