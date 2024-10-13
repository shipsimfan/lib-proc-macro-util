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
     1. Expression
       1. PathExpression
       2. GroupedExpression
       3. ArrayExpression
       4. AwaitExpression
       5. IndexExpression
       6. TupleExpression
       7. TupleIndexingExpression
       8. StructExpression
       9. CallExpression
       10. MethodCallExpression
       11. FieldExpression
       12. ClosureExpression
       13. AsyncBlockExpression
       14. ContinueExpression
       15. BreakExpression
       16. RangeExpression
       17. ReturnExpression
       18. UnderscoreExpression
       19. ConstBlockExpression
       20. UnsafeBlockExpression
       21. LoopExpression
       22. IfExpression
       23. IfLetExpression
       24. MatchExpression
     2. Item
       1. VisItem
         1. Module
         2. ExternCrate
         3. UseDeclaration
         4. Function
         5. TypeAlias
         6. Struct
         7. Enumeration
         8. Union
         9. ConstantItem
         10. StaticItem
         11. Trait
         12. Implementation
         13. ExternBlock
       2. MacroItem
         1. MacroInvocationSemi
         2. MacroRulesDefinition
     3. Statement
       1. Item
       2. LetStatement
       3. ExpressionStatement
     4. Crate
   2. macros
     1. derive
     2. attribute