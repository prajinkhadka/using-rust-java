public class RustJsonParser {
    static {
      
      System.load("/Users/prajin/Downloads/blogs/codee/rust_jni/rust_new/target/x86_64-apple-darwin/release/librust_json_parser.dylib");

    }
    public static native String parseJson(String path);
}
