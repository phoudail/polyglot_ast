import java.io.File;

import javax.naming.Context;
import javax.xml.transform.Source;

import org.graalvm.polyglot.*;


public class JavaTest {
    Context cx0;

    public static void main(String[] args) {
        Context cx1;
        Context cx2 = Context.create();
        File f = new File("TestSamples/pyprint.py");
        Source sourcep = Source.newBuilder("python", f).build();
        try (Context context = Context.create()) {
            context.eval("python", "print('hello')");
            context.eval(sourcep);
            Value bindings = context.getPolyglotBindings();
            bindings.getMember("test");
        }
        cx2.getPolyglotBindings().getMember("null");
    }
}