import java.io.File;

import javax.naming.Context;
import javax.xml.transform.Source;

import org.graalvm.polyglot.Context;
import org.graalvm.polyglot.Value;

public class JavaTest3 {
    public static void main(String[] args) {

        Context cx = Context.create();

        File file1 = new File("TestSamples/import_x.py");
        File file2 = new File("TestSamples/test_pyprint_file.js");

        Source source1 = Source.newBuilder("python", file1).build();
        Source source2 = Source.newBuilder("js", file2).build();

        try (Context context = Context.create()) {
            context.eval(source1);
            context.eval(source2);
            
            Value bindings = context.getPolyglotBindings();
            bindings.getMember("test");
        }
        cx.getPolyglotBindings().getMember("null");
    }
}
