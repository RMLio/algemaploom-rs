package be.ugent.idlab.knows.mappingLoom;

import java.io.IOException;

class Translator implements ITranslator {
	private static native String translate(String input);

	static {
        try {
			String os = System.getProperty("os.name");
			String arch = System.getProperty("os.arch");
            String path;
            if (os.contains("Windows")) {
                path = "/Windows/translator.dll";
            } else if (os.contains("Linux")) {
                path = "/Linux/libtranslator.so";
            } else if (os.contains("Mac")) {
                if (arch.contains("aarch64") || arch.contains("arm")) {
                    path = "/Apple-aarch64/libtranslator.dylib";
                } else {
                    path = "/Apple-x86_64/libtranslator.dylib";
                }
            } else {
                path = "/" + os + "/translator.dll";
            }
            NativeUtils.loadLibraryFromJar(path);
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
