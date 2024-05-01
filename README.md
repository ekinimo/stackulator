# Stackulator
Stackulator is a stack based calculator/programming language bundled with a graphical user interface.
The project is WIP. 

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

#####  List
`List(1 2 3, 3 4 5 add)`
TODO
#####  Set
`Set(1 2 3, 3 4 5 add)`
TODO
#####  Maps
`Map( List(1 2), List(3, 3) List(4 3 5 add) )`
TODO

#### Constructs
Control flow constructs and functions. They generally take something from the stack do something with it and pushes back the result

##### While 
Syntax is `while condition {body}`. Body is evaluated untill condition is met.
Conditions must return a boolean. You can break out of the body using `break` 
- `while true {1 2 3}`
- `while 2 3 leq {1 2 3}`

##### Break
Hands the control to the upper level.

##### Return
Hands the control to the top level.

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
``` 
                    
                    |    1  2               => 3,
                    | 1  1  1               => 5,
                    |      _x when _x 3 geq => true
```
###### Advanced Patterns
TODO
##### Primitive Calls 
###### Arithmetic
- `add`
- `sub` 
- `mult` 
- `div`
###### Boolean
- `and`
- `or`
- `not`
###### Compare
- `eq`
- `geq`
- `leq`
- `ge`
- `le`
###### Conversion
- `i2f` Integer to Float
- `f2i` Float to Integer
###### Quotation related
- `apply`


### Functions
You can define functions by `function_name = body`. you can seperate functions and definitions with `;`.
defined functions can later be called with `function_name`.



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
    

## TODO

- There are currently no checks for multiple variable names in take blocks
- Break should only be possible in while loops (possibly also in take blocks?)
- Chars
- Vectors (mono typed)
- Lists [Partly Done]
- Sets  [Partly Done]
- Maps  [Partly Done]
- structs / enums and match compatability
- interfaces
- VM should run on its own thread. 
- GUI should support saving and opening/appending a session.
- GUI debug. Language Eval trait should have step method
- GUI History bug
- GUI history search
- GUI editable value stack 
- GUI editable definitions
- GUI definition search
- GUI suggest function
- GUI better editing
- Language formatter
- Language pretty print/display

This here is more of a roadmap
- Give language ability to acces the GUI itself.
- Stack size checking (We should have an option of the language where `while` , `if` and `match` respects the stack size ) 
- Statical type system (We should have an option of the language where the program is staticallly typed. We probably will require type signatures at function boundries ) 
