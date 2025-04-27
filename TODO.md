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
         1. StructPattern
         2. TupleStructPattern
         3. TuplePattern
     3. Item
       1. VisItem
         1. TypeAlias
         2. Enumeration
         3. Union
         4. ConstantItem
         5. StaticItem
         6. Trait
         7. Implementation
         8. ExternBlock
       2. MacroItem
         1. MacroRulesDefinition
     4. Statement
       1. LetStatement
     5. Expression
       1. GroupedExpression
       2. ArrayExpression
       3. AwaitExpression
       4. IndexExpression
       5. TupleExpression
       6. TupleIndexingExpression
       7. StructExpression
       8. ClosureExpression
       9. AsyncBlockExpression
       10. ContinueExpression
       11. BreakExpression
       12. RangeExpression
       13. ReturnExpression
       14. UnderscoreExpression
       15. ConstBlockExpression
       16. LoopExpression
       17. IfExpression
       18. IfLetExpression
       19. MatchExpression
     6. Crate
   2. macros
     1. attribute