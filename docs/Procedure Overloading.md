# Procedure Overloading

## Name Mangling

To achive procedure overloading, we have to implement something called `Name Mangling`. This is basically just taking the name of the procedure and messing it up a bunch to make sure that it's unique. The format Real is using is something like `[M]_ReturnType_Name_[PT]`, where `M` is the modifiers, `ReturnType` being the return type (duh), and `PT` being the parameter types.

Example:

```real
public virtual procedure TestProcedure(a: Int, b: Bool) -> Unit = ...
```
The mangled name for that function would be `PV_Unit_TestProcedure_IB`.