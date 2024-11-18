package be.ugent.algemaploom;

public interface ITranslator {
	/**
	 * Translate a mapping into a AlgeMapLoom document.
	 *
	 * @param mapping Mapping to translate in RML or ShExML format.
	 * @param document AlgeMapLoom document generated from mapping
	 */
	String translate_to_document(String document);
}
