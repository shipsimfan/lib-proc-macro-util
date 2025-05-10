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
         1. TypeAlias
         2. Union
         3. ConstantItem
         4. StaticItem
         5. Trait
         6. Implementation
         7. ExternBlock
       2. MacroItem
         1. MacroRulesDefinition
     2. Expression
       1. GroupedExpression
       2. ArrayExpression
       3. AwaitExpression
       4. IndexExpression
       5. TupleExpression
       6. TupleIndexingExpression
       7. StructExpression
       8. ClosureExpression
       9. AsyncBlockExpression
       10. BreakExpression
       11. RangeExpression
       12. ReturnExpression
       13. LoopExpression
         1. Infinite
         2. Predicate
         3. Predicate Pattern
         4. Block
       14. IfExpression
       15. IfLetExpression
       16. MatchExpression
       17. ErrorPropagationExpression
       18. NegationExpression
       19. ArithmeticOrLogicalExpression
       20. ComparisonExpression
       21. LazyBooleanExpression
       22. TypeCastExpression
       23. AssignmentExpression
       24. CompoundAssignmentExpression
     3. Crate
   2. macros
     1. attribute