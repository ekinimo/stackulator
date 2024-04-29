# Stackulator
  Stackulator is a stack based calculator/programming language bundled with a graphical user interface.
## Language
### Expressions
    Syntax of Expressions consists of following atoms and constructs. Stringing together atoms and constructs result in an expression. 
#### Atoms
Atoms semantically pushes themselves on to the value stack.
##### Booleans     
Truth values. can be constructed via `true` `false`
##### Integers         
arbitary precision Integers
- `123124`
##### Floats           
arbitary precision rationals
- `1.2`
            - `23432.564`
##### Stack/Quotation  
Analogous to lambdas. It is a value. You can call it by `apply`
- `[12 3 4]`
  - `[add ]`
#### Constructs
Control flow constructs and functions. They generally take something from the stack do something with it and pushes back the result
##### While 
Syntax is `while condition {body}`. Body is evaluated untill condition is met.
Conditions must return a boolean.
 - `while true {1 2 3}`
 - `while 2 3 leq {1 2 3}`
##### If   
Syntax is `?{body}`. Last value on the stack must be a boolean. if its true body is evaluated.
- `true ?{ 1 2 3 }`
##### Take 
Syntax is `| variables |{ body }`. Variables are of the form `_var` i.e. they have to start with `_`.
            Variables are bound to the end of the stack and can be used in body.
- `| _x _y |{_y _x}`
- `| _x |   { _x _x}`
##### Match 
Syntax is `match (| patterns (when condition)? => body ,)+`. Pattern can be a variable or integer or `_`(don't care).
Patterns will be matched one by one to the top of the stack if they match additionally condiotion after when is checked in that case body is evaluated.
Otherwise next patterns will be tried
- ``` 
                    match
                    |    1  2               => 3,
                    | 1  1  1               => 5,
                    |      _x when _x 3 geq => true
```
##### Primitive Calls :
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

### Functions
You can define functions by `function_name = body`. you can seperate functions and definitions bwith `;`.



### Sample Programs
#### Fibonacci
 ```
 drop = |_x| {};
 
drop2 = |_x _y| { };

dup = |_x| {_x _x} ;

swap = |_x _y|{ _y _x};

fib_step = | _x _y _z |{
  _x _y add 
  _x
  _z 1 sub
 };

fib = | _n |{ 1 1 _n } 
      while dup 1 eq not{
          fib_step
      }
      drop2 ;

0 while dup 100 leq  {
    1 add dup fib swap
} ;
```
## GUI Shortcuts
    | `<Ctr> + <Enter>` | Evaluates the expression|
    | `<F4>`            | Evaluates the expression|
    | `<Ctr> + <Up>`    | cycles through history  |
    | `<Ctr> + <Down>`  | cycles through history  |
    
