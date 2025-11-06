pub fn compile_c(brainfuck_code: String, tape_length: i32) -> String {
    let mut filted_code = String::new();
    for (i, c) in brainfuck_code.chars().enumerate() {
        match c {
            '>' => filted_code.push('+'),
            '<' => filted_code.push('-'),
            '+' => filted_code.push('+'),
            '-' => filted_code.push('-'),
            '.' => filted_code.push('.'),
            ',' => filted_code.push(','),
            '[' => filted_code.push('['),
            ']' => filted_code.push(']'),
            _ => {
                eprintln!("Unexpected character {}:{}", i, c);
            }
        }
    }

    let mut ccode = r#"
        #include <stdio.h>
        #include <stdlib.h>
        #define TAPE_SIZE "#
        .to_string();
    ccode += tape_length.to_string().as_str();
    ccode += "\n";
    ccode += r#"
        int main(){unsigned char tape[TAPE_SIZE] = {0};unsigned char *ptr = tape;
    "#;

    for (i, c) in filted_code.chars().enumerate() {
        match c {
            '>' => ccode.push_str("++ptr;"),
            '<' => ccode.push_str("--ptr;"),
            '+' => ccode.push_str("++*ptr;"),
            '-' => ccode.push_str("--*ptr;"),
            '.' => ccode.push_str("putchar(*ptr);"),
            ',' => ccode.push_str("*ptr = getchar();"),
            '[' => ccode.push_str("while (*ptr)"),
            ']' => ccode.push_str(""),
            _ => {
                eprintln!("Unexpected character {}:{}", i, c);
            }
        }
    }

    ccode.push_str(
        r#"
        return 0;}
    "#,
    );

    ccode
}
