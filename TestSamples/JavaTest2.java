import java.io.File;

import javax.naming.Context;
import javax.xml.transform.Source;

import org.graalvm.polyglot.Context;
import org.graalvm.polyglot.Value;

public class JavaTest2 {
    public static void main(String[] args) {

        Context cx = Context.create();

        File file0 = new File("TestSamples/pyprint.py");
        File file1 = file0;
        File file2 = new File("TestSamples/export_x.py");
        File file3 = new File("TestSamples/JavaTest.java");

        Source source1 = Source.newBuilder("python", file1).build();
        Source source2 = Source.newBuilder("python", file2).build();
        Source source3 = Source.newBuilder("java", file3).build();

        try (Context context = Context.create()) {
            context.eval("python", "print('hello')");
            context.eval(source1);
            context.eval(source2);
            context.eval(source3);
            
            Value bindings = context.getPolyglotBindings();
            bindings.getMember("test");
        }
    }
    static Source source0 = Source.newBuilder("python", new File("TestSamples/pyprint.py")).build();
    static Source source1 = Source.newBuilder("python", new File("TestSamples/pyprint.py")).build();
    static Source src = source1;
    static Source src1 = source0.build();

}
