package be.ugent.idlab.knows.mappingLoom;

public interface ITranslator {
	/**
	 * Translate a mapping into a MappingLoom document.
	 *
	 * @param document Mapping to translate in RML or ShExML format.
     * @return The MappingLoom plan
	 */
	String translate_to_document(String document);

	/**
	 * A default instance of the translator.
     *
     * @return A new ITranslator instance
	 */
	static ITranslator getInstance() {
		return new Translator();
	}
}
