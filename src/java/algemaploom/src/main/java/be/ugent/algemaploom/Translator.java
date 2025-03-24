package be.ugent.algemaploom;

import java.io.IOException;

class Translator implements ITranslator {
	private static native String translate(String input);

	static {
		//System.load(Paths.get("/home/dylan/Projects/algemaploom-rs/target/release/libltranslator.so").toAbsolutePath().toString());
		//System.loadLibrary("ltransator");
        try {
			String os = System.getProperty("os.name");
            NativeUtils.loadLibraryFromJar("/" + os + "/libltranslator.so");
        } catch (IOException e) {
            throw new RuntimeException("Cannot dynamically load Rust library: " + e);
        }
    }

	Translator() {

	}

	@Override
	public String translate_to_document(String document) {
		return translate(document);
	}
}
