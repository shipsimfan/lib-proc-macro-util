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
       1. TypeCastExpression
       2. IndexExpression
       3. AwaitExpression
       4. TupleIndexingExpression
       5. NegationExpression
       6. ArithmeticOrLogicalExpression
       7. LazyBoolExpression
       8. AssignmentExpression
       9. CompoundAssignmentExpression
       10. ClosureExpression
       11. TupleExpression
       12. RangeExpression
       13. StructExpression
     3. Crate