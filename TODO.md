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
     1. WhereClause
     2. Pattern
       1. PatternWithoutRange
         1. LiteralPattern
         2. IdentifierPattern
         3. WildcardPattern
         4. RestPattern
         5. ReferencePattern
         6. StructPattern
         7. TupleStructPattern
         8. TuplePattern
         9. GroupedPattern
         10. SlicePattern
         11. PathPattern
         12. MacroInvocation
       2. PatternNoTopAlt
         1. RangePattern
     3. Item
       1. VisItem
         1. Function
         2. TypeAlias
         3. Struct
         4. Enumeration
         5. Union
         6. ConstantItem
         7. StaticItem
         8. Trait
         9. Implementation
         10. ExternBlock
       2. MacroItem
         1. MacroRulesDefinition
     4. Statement
       1. Item
       2. LetStatement
       3. ExpressionStatement
     5. Expression
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
     6. Crate
   2. macros
     1. derive
     2. attribute