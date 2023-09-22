import java.io.File;

import javax.naming.Context;
import javax.xml.transform.Source;

import org.graalvm.polyglot.*;


public class JavaTest {
    public static void main(String[] args) {
        try (Context context = Context.create()) {
            context.eval("python", "print('hello')");
        }
    }
}