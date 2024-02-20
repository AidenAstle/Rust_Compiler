### Grammar Key
```
'' = specific string of characters
[] = optional
{} = at least 1
| = or
() = grouping
```



## GRAMMAR FOR CUT 1 : Conditionals
```
conditional_expression = conditional_L1;
conditional_L1 = conditional_L2, [{conditional_L1_infix}];
conditional_L1_infix = operator, expression;
conditional_L2 = expression, operator, expression;
operator = '>' | '<' | '>=' | '<=' | '==' | '!=';
```

**Valid Expressions**  
```
5 != 11
10 + 2 > 10 == false
h + 2 <= g - 5
```  
**Invalid Expressions**  
```
5 != false
true == 10 > 5
16 > true
```


    

## GRAMMAR FOR CUT 2 : If Statements
```
if_statement = if_case, [{else_if_case}], else_case;
if_case = 'if (', (conditional_expression | boolean | identifier), '){', {statement}, '}';
else_if_case = 'else if (', (conditional_expression | boolean | identifier), '){', {statement}, '}';
else_case = 'else {', {statement}, '}';
```

**Valid Expressions**  
```
if (true){return 12;} else {return 24;}
if (x>y){return true;} else if (x<y){return false;} else if (x == z){return true;} else {return true;}
if (12+h > 2){ return h*2;} else {return 3;};
```  
**Invalid Expressions**  
```
if (true){return false;} else {return 8;}
if (false){return 1;} else if (true){return 2;}
if (5 > c)(return 11;) else {let c = 5;}
```