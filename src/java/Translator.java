import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.IOException;

class Translator {
    private static native String translate(String input);

    static {
        System.loadLibrary("ltranslator");
    }

    public static void main(String[] args) throws IOException {
        BufferedReader in = null;
        String mapping = new String();
        try {
            in = new BufferedReader(new InputStreamReader(System.in));
            String line;
            while ((line = in.readLine()) != null) {
                mapping = mapping.concat(line);
            }
        }
        catch (IOException e) {
            System.err.println("Reading mapping from stdin failed: " + e);
            System.exit(-1);
        }
        finally {
            if (in != null) {
                in.close();
            }
        }

        String translation = Translator.translate(mapping);
        System.out.println(translation);
    }
}
