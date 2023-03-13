import polyglot

x = 42
polyglot.export_value(name="x", value=x)
polyglot.eval(path="impport_x.py", language="python")