import polyglot

x = 42
polyglot.export_value(name="x", value=x)
polyglot.eval(path="import_x.py", language="python")