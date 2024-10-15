# ToDo List
 1. Refactor/Improvement
  *Things to think about:*
   1. Debug
   2. Clone
   3. PartialEq + Eq
   4. Public member(s)
   5. Ownership and borrowing
   6. Simple new function(s)
   7. Clear utility functions
   8. Parse
   9. ToTokens
   10. Deref
   11. From<>
   12. Into<>
   13. Error messages and spans

  *Things to implement:*
   1. ast
     1. Item
       1. VisItem
         1. ExternCrate
         2. UseDeclaration
         3. Function
         4. TypeAlias
         5. Struct
         6. Enumeration
         7. Union
         8. ConstantItem
         9. StaticItem
         10. Trait
         11. Implementation
         12. ExternBlock
       2. MacroItem
         1. MacroInvocationSemi
         2. MacroRulesDefinition
     2. Statement
       1. Item
       2. LetStatement
       3. ExpressionStatement
     3. Expression
       1. GroupedExpression
       2. ArrayExpression
       3. AwaitExpression
       4. IndexExpression
       5. TupleExpression
       6. TupleIndexingExpression
       7. StructExpression
       8. CallExpression
       9. MethodCallExpression
       10. FieldExpression
       11. ClosureExpression
       12. AsyncBlockExpression
       13. ContinueExpression
       14. BreakExpression
       15. RangeExpression
       16. ReturnExpression
       17. UnderscoreExpression
       18. ConstBlockExpression
       19. UnsafeBlockExpression
       20. LoopExpression
       21. IfExpression
       22. IfLetExpression
       23. MatchExpression
     4. Crate
   2. macros
     1. derive
     2. attribute