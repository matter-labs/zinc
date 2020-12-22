# Dependency system

Zargo provides the possibility to use other Zinc projects as dependencies.

To use a dependency, specify its name and version in the `dependencies` section
of your `Zargo.toml` project manifest:

```toml,no_run,noplaypen
[project]
name = 'caller'
type = 'contract'
version = '0.1.0'

[dependencies]
callee = '0.1.0'
```

Then, the dependency project will be available through the main one as an ordinar module.
So easy!

```rust,no_run,noplaypen
use callee::Callee;

contract Caller {
    pub value: u64;

    pub fn new(value: u64) -> Self {
        Self {
            value: value,
        }
    }

    pub fn create_and_transfer(mut self) {
        // creates an instance of contract `Callee`
        let mut instance = Callee::new(self.value / 2);
        
        // sends some tokens to the newly created instance
        self.transfer(instance.address, 0x0 as u160, 0.1_E18 as u248);
        
        // sends half of the tokens back to the creator
        instance.transfer(self.address, 0x0 as u160, 0.05_E18 as u248);
    }
}
```

## Uploading a project

To upload your project to the Zandbox database, simply use the `zargo upload`
command. The project name and version must be unique. To check which ones are
already occupied, use this command:

```bash,no_run,noplaypen
zargo download --list
```

## Downloading a project

Usually, all the project dependencies are downloaded by default and stored
in the `target/deps` directory relative to the main project root.

However, sometimes you need to download some project of yours or somebody else's
to make useful changes and tweaks. To do that, use the following command:

```bash,no_run,noplaypen
zargo download --name callee --version 0.1.0
```
