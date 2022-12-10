# Normilize files

This is a rust command that looks into a directory, and normilizes files. In
this particular example, it changes all names to be snake case.

## Example

Given the following directory: 

```bash
ls 

HelloMyNameIs.js
homework from school.docx
index.html
my_cat.rs
```

The command would normilized as following:

```bash
normilize_files .
ls

hello_my_name_is.js
homework_from_school.docx
index.html
my_cat.rs
```
