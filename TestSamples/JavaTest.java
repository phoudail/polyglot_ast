import org.graalvm.polyglot.*;


public class JavaTest {
    Context cx0;

    public static void main(String[] args) {
        Context cx1;
        Context cx2 = Context.create();
        try (Context context = Context.create()) {
            context.eval("python", "print('hello')");
            Value bindings = context.getPolyglotBindings();
            bindings.getMember("test");
        }
        cx2.getPolyglotBindings().getMember("null");
    }
}