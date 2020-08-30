# rust_config_file

Cool Config File Idea I Had That I'm Programming As A First Foray Into Rust.

## Usage
### Windows (cmd.exe)
```
> .\PATH\TO\FILE\config-file.exe file.cfg
```

### Linux/MacOSX
```sh
$ chmod +x config-file
$ ./config-file file.cfg
```

### Flags
```
-o : specify output file
  example:
    config-file file.cfg -o file.json
```

## Creating Definitions

config file format
```
first_indentifier = 10
second_identifier = "string"
third_identifier = true
```

jsonified (resulting json is not prettyfied)
```json
{
  "first_identifier": 10,
  "second_identifier": "string",
  "third_identifier": true
}
```

## Creating Objects

config file format
```
Goblin {
  name: "A Wonderful Goblin",
  health: 10,
}
```

jsonified
```
{
  "Goblin": {
    "name": "A Wonderful Goblin",
    "health": 10
  }
}
```

## Specifying Types

config file format
```
type Creature {
  name: "",
  health: 10,
  strength: 5,
  defense: 5,
}

Goblin : Creature {
  name: "A Wonderful Goblin",
}
```

jsonified
```
{
  "Goblin": {
    name: "A Wonderful Goblin",
    health: 10,
    strength: 5,
    defense: 5
  }
}
```

## Specifying a type for the rest of the file
Once a type label is set, the type cannot be changed for the rest of the file

config file format
```
type Creature {
  name: "",
  health: 10,
  strength: 5,
  defense: 5,
}

[Creature]
Goblin {
  name: "A Wonderful Goblin",
}

Vampire {
  name: "A Scary Vampire",
}
```

jsonified
```
{
  "Goblin": {
    name: "A Wonderful Goblin",
    health: 10,
    strength: 5,
    defense: 5
  },
  "Vampire": {
    name: "A Scary Vampire",
    health: 10,
    strength: 5,
    defense: 5
  }
}
```
