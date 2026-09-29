# Things to do

## Basic Features

- [X] Allow creating a project from a directory;
- [ ] Move walking the file system to somewhere within `proj`;
    - Use `walkdir` probably!
    - [ ] Traits for which folders to visit and what files to consider;
- [X] Allow heterogeneous modules (useful for config files);
- [X] Allow retrieving information based on cursor position;
    - [ ] Return an item path, as well as information about the selected node;
    - Item paths would be like `foo::bar::baz` identifying functions within functions or fields within structs or subheadings within headings etc;
- [ ] Maybe use `cstree` for syntax trees?

## Super Powers

- [ ] Backlinks (get all modules that refer to an item);
    - [ ] Operations with references (replace references to an items with references to another item);