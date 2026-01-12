import json
import sys
import os
from collections import defaultdict

def load_api(path):
    with open(path, 'r') as f:
        return json.load(f)

def compare(bevy, autozig):
    report = {}
    total_missing = 0
    total_bevy = 0

    for category in ["structs", "enums", "traits", "functions", "types"]:
        # Index AutoZig items by name for quick lookup
        autozig_items = {item["name"] for item in autozig.get(category, [])}
        bevy_items = bevy.get(category, [])
        
        missing = []
        for item in bevy_items:
            if item["name"] not in autozig_items:
                missing.append(item)
        
        report[category] = {
            "total_bevy": len(bevy_items),
            "total_autozig": len(autozig_items),
            "missing_count": len(missing),
            "missing_items": missing
        }
        total_missing += len(missing)
        total_bevy += len(bevy_items)

    return report, total_missing, total_bevy

def generate_markdown(report, total_missing, total_bevy):
    md = "# Bevy ECS vs AutoZig ECS API Comparison\n\n"
    md += f"**Total Bevy Public Items:** {total_bevy}\n"
    md += f"**Total Missing Items:** {total_missing}\n"
    completion = 0
    if total_bevy > 0:
        completion = (total_bevy - total_missing) / total_bevy * 100
    md += f"**Completion:** {completion:.2f}%\n\n"

    # Aggregate all missing items by path
    missing_by_path = defaultdict(list)
    
    for category, data in report.items():
        if data["missing_count"] == 0:
            continue
            
        for item in data["missing_items"]:
            # Format: Struct `Name`
            desc = f"{category[:-1 if category.endswith('s') else 0].capitalize()} `{item['name']}`"
            path = item["path"]
            # Clean up path to make it look like a module
            # e.g., world/mod.rs -> bevy_ecs::world
            # simple tweak: assume relative path from src
            
            missing_by_path[path].append(desc)

    # Sort paths
    sorted_paths = sorted(missing_by_path.keys())
    
    for path in sorted_paths:
        items = missing_by_path[path]
        md += f"### `{path}`\n\n"
        for item in sorted(items):
            md += f"- [ ] {item}\n"
        md += "\n"

    return md

def main():
    if len(sys.argv) < 3:
        print("Usage: python3 compare_ecs.py <bevy_json> <autozig_json>")
        sys.exit(1)
        
    bevy_path = sys.argv[1]
    autozig_path = sys.argv[2]
    
    bevy_api = load_api(bevy_path)
    autozig_api = load_api(autozig_path)
    
    report, missing, total = compare(bevy_api, autozig_api)
    
    print(generate_markdown(report, missing, total))

if __name__ == "__main__":
    main()
