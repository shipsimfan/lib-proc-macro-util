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
       1. IndexExpression
       2. AwaitExpression
       3. TupleIndexingExpression
       4. NegationExpression
       5. ArithmeticOrLogicalExpression
       6. LazyBoolExpression
       7. AssignmentExpression
       8. CompoundAssignmentExpression
       9. ClosureExpression
       10. TupleExpression
       11. RangeExpression
       12. StructExpression
     3. Crate