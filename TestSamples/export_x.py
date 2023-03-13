import polyglot

x = 42
polyglot.export_value(name="x", value=x)
polyglot.eval(path="test_b.py", language="python")