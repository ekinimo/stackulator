# Stackulator
  Stackulator is a stack based calculator/programming language bundled with a graphical user interface.

## Expressions
 
* Integers        : 
     arbitary precision Integers
                - `123124`
* Floats          : 
     arbitary precision rationals
                - `1.2`
                - `23432.564`
* Stack/Quotation : 
     Analogous to lambdas. It is a value. You can call it by `apply`
                - `[12 3 4]`
                - `[add ]`
* While :
     Syntax is `while condition {body}`. Body is evaluated untill condition is met.
     Conditions must return a boolean.
               - `while true {1 2 3}`
               - `while 2 3 leq {1 2 3}`
* If   :
    Syntax is `?{body}`. Last value on the stack must be a boolean. if its true body is evaluated.
               - `true ?{ 1 2 3 }`
* Take :
    Syntax is `| variables |{ body }`. Variables are of the form `_var` i.e. they have to start with `_`.
    Variables are bound to the end of the stack and can be used in body.
               - `| _x _y |{_y _x}`
               - `| _x |   { _x _x}`
* Match :
    Syntax is `match (| patterns (when condition)? => body ,)+`. Pattern can be a variable or integer or `_`(don't care).
               - ``` 
                    match
                    |    1  2               => 3,
                    | 1  1  1               => 5,
                    |      _x when _x 3 geq => true
                 ```
* Primitive Calls :
** Arithmetic
    - `add`
    - `sub` 
    - `mult` 
    - `div`
** Boolean
    - `and`
    - `or`
    - `not`
** Compare
    - `eq`
    - `geq`
    - `leq`
    - `ge`
    - `le`
** Conversion
    - `i2f` Integer to Float
    - `f2i` Float to Integer
** Stack related
    - `apply`

## Functions
You can define functions by 
`function_name = body`
