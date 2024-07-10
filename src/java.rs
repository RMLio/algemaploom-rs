use jni::JNIEnv;
use jni::objects::{JClass, JString};
use crate::api::process_one_str;

#[no_mangle]
pub extern "system" fn Java_Translator_translate<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    let mapping: String = env
        .get_string(&input)
        .expect("Failed to retrieve mapping as string from Java")
        .into();

    let translated = process_one_str(mapping.clone().as_str());

    let output = env
        .new_string(translated)
        .expect("Couldn't create translated mapping as Java string!");
    output
}
