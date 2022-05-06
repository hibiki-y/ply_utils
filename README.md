# ply_utils
This is a collection of utility function for ply.

## Usage
```
cargo run -- SUBCOMMAND
```

## SUBCOMMANDS
### help
Prints the help of the given subcommands
### cut
Cut out ascii ply property
- you need option `-p cut_property_names(if you choice multiple properties, need space) -i input_path -o output_path`
### decode
Decode binary ply property
- you need option `-i input_path -o output_path`
## Example
```
cargo run -- help
```