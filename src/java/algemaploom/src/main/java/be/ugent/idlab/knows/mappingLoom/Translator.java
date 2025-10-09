package be.ugent.idlab.knows.mappingLoom;

import java.io.IOException;

class Translator implements ITranslator {
	private static native String translate(String input);

	static {
        try {
			String os = System.getProperty("os.name");
            if (os.contains("Windows")) {
                os = "Windows";
            }
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
