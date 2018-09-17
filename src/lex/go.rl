%%{
    machine go;

    newline         = '\n';
    unicode_letter  = any - newline; # TODO

    # Letters and digits
    letter          = unicode_letter | '_';
    decimal_digit   = '0'..'9';
    octal_digit     = '0'..'7';
    hex_digit       = '0'..'9' | 'A'..'F' | 'a'..'f';

    # Integer literals
    decimal_lit     = '1'..'9' decimal_digit*;
    octal_lit       = '0' octal_digit*;
    hex_lit         = '0' [xX] hex_digit+;
    int_lit         = decimal_lit | octal_lit | hex_lit;

    # Floating point literals
    decimals        = decimal_digit+;
    exponent        = [eE] [\-+]? decimals;
    float_lit       = (decimals '.' decimals? exponent?)
                    | decimals exponent
                    | ('.' decimals exponent?)
                    ;

    # Imaginary literals
    imaginary_lit   = (decimals | float_lit) 'i';

    # --

    main := int_lit | float_lit | imaginary_lit;
}%%
