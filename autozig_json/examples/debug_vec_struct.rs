use autozig_json::{parse, TapeRef, AutoDeserialize, NodeType};

#[derive(Debug)]
struct Item {
    name: String,
}

impl AutoDeserialize for Item {
    fn from_tape(json: &str, tape: &TapeRef, root_idx: usize) -> autozig_json::Result<Self> {
        eprintln!("Item::from_tape called with idx={}", root_idx);
        
        let root = tape.get(root_idx);
        eprintln!("  root.tag={:?}, root.child={}", root.tag, root.child);
        
        if root.tag != NodeType::Object {
            return Err(autozig_json::Error::TypeMismatch { expected: "object", found: "other" });
        }
        
        let mut _field_name: Option<String> = None;
        
        let mut key_idx = root.child as usize;
        let mut iterations = 0;
        while key_idx != 0 && iterations < 100 {
            iterations += 1;
            let key_node = tape.get(key_idx);
            let val_idx = key_node.child as usize;
            
            eprintln!("  key_idx={}, val_idx={}, key_node.next={}", key_idx, val_idx, key_node.next);
            
            if let Some(key) = tape.get_str(json, key_node) {
                eprintln!("  key=\"{}\"", key);
                if key == "name" {
                    _field_name = Some(<String as AutoDeserialize>::from_tape(json, tape, val_idx)?);
                }
            }
            
            key_idx = key_node.next as usize;
        }
        
        if iterations >= 100 {
            eprintln!("  WARNING: hit iteration limit!");
        }
        
        Ok(Item {
            name: _field_name.ok_or_else(|| autozig_json::Error::KeyNotFound { key: "name".to_string() })?,
        })
    }
}

#[derive(Debug)]
struct Container {
    items: Vec<Item>,
}

impl AutoDeserialize for Container {
    fn from_tape(json: &str, tape: &TapeRef, root_idx: usize) -> autozig_json::Result<Self> {
        eprintln!("Container::from_tape called with idx={}", root_idx);
        
        let root = tape.get(root_idx);
        if root.tag != NodeType::Object {
            return Err(autozig_json::Error::TypeMismatch { expected: "object", found: "other" });
        }
        
        let mut _field_items: Option<Vec<Item>> = None;
        
        let mut key_idx = root.child as usize;
        while key_idx != 0 {
            let key_node = tape.get(key_idx);
            let val_idx = key_node.child as usize;
            
            if let Some(key) = tape.get_str(json, key_node) {
                eprintln!("Container: key=\"{}\", val_idx={}", key, val_idx);
                if key == "items" {
                    _field_items = Some(<Vec<Item> as AutoDeserialize>::from_tape(json, tape, val_idx)?);
                }
            }
            
            key_idx = key_node.next as usize;
        }
        
        Ok(Container {
            items: _field_items.ok_or_else(|| autozig_json::Error::KeyNotFound { key: "items".to_string() })?,
        })
    }
}

fn main() {
    let json = r#"{ "items": [ { "name": "A" } ] }"#;
    eprintln!("Parsing JSON: {}", json);
    
    let tape = TapeRef::parse(json).unwrap();
    eprintln!("Tape has {} nodes", tape.len());
    
    // Debug: print all nodes
    for i in 0..tape.len() {
        let n = tape.get(i);
        eprintln!("Node[{}]: tag={:?}, start={}, len={}, next={}, child={}", 
            i, n.tag, n.start, n.len, n.next, n.child);
    }
    
    match Container::from_tape(json, &tape, 0) {
        Ok(c) => println!("Success: {:?}", c),
        Err(e) => println!("Error: {}", e),
    }
}
