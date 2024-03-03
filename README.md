# Command line utilities using rust

This is a beginner project in rust programming language, it consists of some basic command line utilities:

1. echo
2. cat
3. ls
4. find
5. grep

## Usage

cargo run:

### Echo a text to terminal:

```
echo your text
```

### Read the content of file:

```
cat /path
```

### Concatenate two files into one file:

```
cat pathOfFirstFile pathOfSecondFile targetFile
```

### List directories:

```
ls /path
```

### Find a file by name:

```
find startDirectory -type f -name filename
```

### Find a directory by name:

```
find startDirectory -type d -name dirname
```

### Find an expression in a file and echo the line:

```
grep expression filename
```

### Find an expression with case insensitivity in a file and echo the line:

```
grep expression filename -i
```
