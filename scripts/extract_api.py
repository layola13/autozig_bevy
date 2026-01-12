import os
import re
import argparse
import json

def extract_public_items(root_dir):
    items = {
        "structs": [],
        "enums": [],
        "traits": [],
        "types": [],
        "functions": [],
        "consts": []
    }

    # Regex patterns for public items
    p_struct = re.compile(r'pub\s+struct\s+([a-zA-Z_][a-zA-Z0-9_]*)')
    p_enum = re.compile(r'pub\s+enum\s+([a-zA-Z_][a-zA-Z0-9_]*)')
    p_trait = re.compile(r'pub\s+trait\s+([a-zA-Z_][a-zA-Z0-9_]*)')
    p_type = re.compile(r'pub\s+type\s+([a-zA-Z_][a-zA-Z0-9_]*)')
    p_fn = re.compile(r'pub\s+(?:unsafe\s+|async\s+|const\s+|extern\s+(?:"[a-zA-Z]+" )?)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)')
    p_const = re.compile(r'pub\s+const\s+([a-zA-Z_][a-zA-Z0-9_]*)')

    for root, dirs, files in os.walk(root_dir):
        for file in files:
            if not file.endswith(".rs"):
                continue
            
            path = os.path.join(root, file)
            # Create a relative module path style string
            rel_path = os.path.relpath(path, root_dir)
            
            with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                
                # Remove comments to avoid false positives
                content = re.sub(r'//.*', '', content)
                content = re.sub(r'/\*.*?\*/', '', content, flags=re.DOTALL)

                def add_items(category, pattern):
                    for name in pattern.findall(content):
                        items[category].append({"name": name, "path": rel_path})

                add_items("structs", p_struct)
                add_items("enums", p_enum)
                add_items("traits", p_trait)
                add_items("types", p_type)
                add_items("functions", p_fn)
                add_items("consts", p_const)

    # Sort by name
    for k in items:
        items[k].sort(key=lambda x: x["name"])

    return items

def main():
    parser = argparse.ArgumentParser(description="Extract public API from Rust source")
    parser.add_argument("dir", help="Source directory to scan")
    parser.add_argument("--output", help="Output JSON file")
    args = parser.parse_args()

    api = extract_public_items(args.dir)
    
    if args.output:
        with open(args.output, 'w') as f:
            json.dump(api, f, indent=2)
    else:
        print(json.dumps(api, indent=2))

if __name__ == "__main__":
    main()
