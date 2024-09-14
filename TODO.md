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
   1. tokens
     1. Keyword
     2. Punctuations
   2. to_tokens
   3. ast
     1. SimplePath
     2. DelimTokenTree
     3. OuterAttribute
     4. InnerAttribute
     5. Visibility
     6. Expression
       1. ExpressionWithoutBlock
         1. LiteralExpression
         2. PathExpression
         3. GroupedExpression
         4. ArrayExpression
         5. AwaitExpression
         6. IndexExpression
         7. TupleExpression
         8. TupleIndexingExpression
         9. StructExpression
         10. CallExpression
         11. MethodCallExpression
         12. FieldExpression
         13. ClosureExpression
         14. AsyncBlockExpression
         15. ContinueExpression
         16. BreakExpression
         17. RangeExpression
         18. ReturnExpression
         19. UnderscoreExpression
         20. MacroInvocation
       2. ExpressionWithBlock
         1. BlockExpression
         2. ConstBlockExpression
         3. UnsafeBlockExpression
         4. LoopExpression
         5. IfExpression
         6. IfLetExpression
         7. MatchExpression
     7. Item
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
     8. Crate
   5. macros
     1. function
     2. derive
     3. attribute
     4. to_tokens