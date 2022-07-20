# create-idea-module

Sometimes Intellij will mess up its own module structure. This tool is designed to help you fix that. It will create a module for you and add it to the correct place in the module structure.

```
USAGE:
    create-idea-module [OPTIONS]

OPTIONS:
    -h, --help                                   Print help information
    -n, --module-name <MODULE_NAME>
    -o, --output-directory <OUTPUT_DIRECTORY>
    -V, --version                                Print version
```

# Example

```
create-idea-module -o ../some-repo -n some-repo
```

For a project named `vanityeth` this will create the idea project directory `../vanityeth/.idea` as well as the module metadata files needed to give Intellij the correct module structure.

# Future work

- Make this a little more flexible—not every project is a rust module.
- Multiple modules per project.
- Specify module files location separate from project directory.
